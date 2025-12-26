use crate::error::{Result, WebhookError};
use crate::multipart::MultipartBuilder;
use crate::models::{Attachment, MessageBuilder, WebhookResponse};
use http_body_util::{BodyExt, Full};
use hyper::body::Bytes;
use hyper::client::conn::http1::handshake;
use hyper::http::{header, Method, Request, Uri};
use hyper_util::rt::TokioIo;
use tokio::net::TcpStream;
use tokio_native_tls::TlsConnector;
use native_tls::TlsConnector as NativeTlsConnector;

const USER_AGENT: &str = "rs-hook (https://github.com/rs-hook, 0.1.0)";

fn tls_error<E: std::fmt::Display>(err: E) -> WebhookError {
    WebhookError::TokioTls(err.to_string())
}

#[derive(Debug, Clone)]
pub struct Webhook {
    url: String,
    timeout: Option<u64>,
}

impl Webhook {
    pub fn new(url: impl Into<String>) -> Result<Self> {
        let url = url.into();
        if !url.contains("discord.com/api/webhooks/")
            && !url.contains("discordapp.com/api/webhooks/")
        {
            return Err(WebhookError::InvalidUrl);
        }
        Ok(Self {
            url,
            timeout: None,
        })
    }

    pub fn with_timeout(mut self, seconds: u64) -> Self {
        self.timeout = Some(seconds);
        self
    }

    pub async fn send(&self, message: MessageBuilder) -> Result<WebhookResponse> {
        self.send_internal(message, None).await
    }

    pub async fn send_with_attachments(
        &self,
        message: MessageBuilder,
        attachments: Vec<Attachment>,
    ) -> Result<WebhookResponse> {
        self.send_internal(message, Some(attachments)).await
    }

    async fn send_internal(
        &self,
        message: MessageBuilder,
        attachments: Option<Vec<Attachment>>,
    ) -> Result<WebhookResponse> {
        let message = message.build()?;

        let uri: Uri = self.url.parse().map_err(|_| WebhookError::InvalidUrl)?;

        let (body, content_type) = if let Some(attachments) = attachments {
            let mut builder = MultipartBuilder::new();
            builder = builder.add_json("payload_json".to_string(), &message)?;

            for (idx, attachment) in attachments.into_iter().enumerate() {
                builder = builder.add_attachment(idx, attachment)?;
            }

            builder.build()?
        } else {
            let json = serde_json::to_string(&message)?;
            (json.into_bytes(), "application/json".to_string())
        };

        let host = uri
            .host()
            .ok_or_else(|| WebhookError::Request("Missing host".to_string()))?
            .to_string();

        let port = uri.port_u16().unwrap_or(443);
        let addr = format!("{}:{}", host, port);

        let stream = TcpStream::connect(&addr).await?;
        let tls_connector = NativeTlsConnector::new()?;
        let connector = TlsConnector::from(tls_connector);
        let tls_stream = connector
            .connect(&host, stream)
            .await
            .map_err(tls_error)?;
        let io = TokioIo::new(tls_stream);

        let (mut sender, conn) = handshake(io).await?;
        tokio::spawn(async move {
            if let Err(err) = conn.await {
                eprintln!("Connection error: {:?}", err);
            }
        });

        let full_body = Full::new(Bytes::from(body));
        let req = Request::builder()
            .uri(uri)
            .method(Method::POST)
            .header(header::USER_AGENT, USER_AGENT)
            .header(header::HOST, host)
            .header(header::CONTENT_TYPE, content_type)
            .body(full_body)
            .map_err(|e| WebhookError::Request(e.to_string()))?;

        let resp = sender.send_request(req).await?;

        let status = resp.status();
        let body = resp.into_body();
        let body_bytes = body.collect().await?.to_bytes();
        let body_string = String::from_utf8_lossy(&body_bytes).to_string();

        if !status.is_success() {
            return Err(WebhookError::Status {
                status,
                body: body_string,
            });
        }

        Ok(WebhookResponse {
            status_code: status.as_u16(),
            body: body_string,
        })
    }

    pub async fn execute(&self, wait: bool) -> Result<WebhookResponse> {
        let url = if wait {
            format!("{}?wait=true", self.url)
        } else {
            self.url.clone()
        };

        let uri: Uri = url.parse().map_err(|_| WebhookError::InvalidUrl)?;

        let host = uri
            .host()
            .ok_or_else(|| WebhookError::Request("Missing host".to_string()))?
            .to_string();

        let port = uri.port_u16().unwrap_or(443);
        let addr = format!("{}:{}", host, port);

        let stream = TcpStream::connect(&addr).await?;
        let tls_connector = NativeTlsConnector::new()?;
        let connector = TlsConnector::from(tls_connector);
        let tls_stream = connector
            .connect(&host, stream)
            .await
            .map_err(tls_error)?;
        let io = TokioIo::new(tls_stream);

        let (mut sender, conn) = handshake(io).await?;
        tokio::spawn(async move {
            if let Err(err) = conn.await {
                eprintln!("Connection error: {:?}", err);
            }
        });

        let full_body = Full::new(Bytes::from("{}"));
        let req = Request::builder()
            .uri(uri)
            .method(Method::POST)
            .header(header::USER_AGENT, USER_AGENT)
            .header(header::HOST, host)
            .header(header::CONTENT_TYPE, "application/json")
            .body(full_body)
            .map_err(|e| WebhookError::Request(e.to_string()))?;

        let resp = sender.send_request(req).await?;

        let status_code = resp.status().as_u16();
        let body = resp.into_body();
        let body_bytes = body.collect().await?.to_bytes();
        let body_string = String::from_utf8_lossy(&body_bytes).to_string();

        Ok(WebhookResponse {
            status_code,
            body: body_string,
        })
    }
}
