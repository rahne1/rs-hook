use crate::models::Attachment;
use std::borrow::Cow;
use std::fs::File;
use std::io::Read;

pub struct MultipartBuilder {
    parts: Vec<(String, MultipartPart)>,
}

pub enum MultipartPart {
    String(String),
    File(AttachmentFile),
}

pub struct AttachmentFile {
    filename: String,
    content: Vec<u8>,
    mime_type: Option<String>,
}

impl MultipartBuilder {
    pub fn new() -> Self {
        Self { parts: Vec::new() }
    }

    pub fn add_json<T: serde::Serialize>(mut self, name: String, value: &T) -> crate::error::Result<Self> {
        let json = serde_json::to_string(value)?;
        self.parts.push((name, MultipartPart::String(json)));
        Ok(self)
    }

    pub fn add_attachment(mut self, index: usize, attachment: Attachment) -> crate::error::Result<Self> {
        let mut file = File::open(&attachment.path)?;
        let mut content = Vec::new();
        file.read_to_end(&mut content)?;

        let filename = attachment
            .path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("attachment")
            .to_string();

        let mime_type = mime_guess::from_path(&attachment.path)
            .first_raw()
            .map(String::from);

        self.parts.push((
            format!("files[{}]", index),
            MultipartPart::File(AttachmentFile {
                filename,
                content,
                mime_type,
            }),
        ));

        Ok(self)
    }

    pub fn build(self) -> crate::error::Result<(Vec<u8>, String)> {
        let boundary = generate_boundary();
        let mut body = Vec::new();

        for (name, part) in self.parts {
            write_boundary(&mut body, &boundary)?;
            write_part_header(&mut body, &name, &part)?;

            match part {
                MultipartPart::String(s) => {
                    body.extend_from_slice(s.as_bytes());
                }
                MultipartPart::File(file) => {
                    body.extend_from_slice(&file.content);
                }
            }
            body.extend_from_slice(b"\r\n");
        }

        body.extend_from_slice(b"--");
        body.extend_from_slice(boundary.as_bytes());
        body.extend_from_slice(b"--\r\n");

        let content_type = format!("multipart/form-data; boundary={}", boundary);
        Ok((body, content_type))
    }
}

fn generate_boundary() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    format!("DiscordWebhookBoundary{}", timestamp)
}

fn write_boundary<W: std::io::Write>(writer: &mut W, boundary: &str) -> std::io::Result<()> {
    writer.write_all(b"--")?;
    writer.write_all(boundary.as_bytes())?;
    writer.write_all(b"\r\n")?;
    Ok(())
}

fn write_part_header<W: std::io::Write>(
    writer: &mut W,
    name: &str,
    part: &MultipartPart,
) -> std::io::Result<()> {
    match part {
        MultipartPart::String(_) => {
            write!(
                writer,
                "Content-Disposition: form-data; name=\"{}\"\r\n\r\n",
                escape_form_data(name)
            )?;
        }
        MultipartPart::File(file) => {
            write!(
                writer,
                "Content-Disposition: form-data; name=\"{}\"; filename=\"{}\"\r\n",
                escape_form_data(name),
                escape_filename(&file.filename)
            )?;
            if let Some(mime) = &file.mime_type {
                writeln!(writer, "Content-Type: {}", mime)?;
            }
            writeln!(writer)?;
        }
    }
    Ok(())
}

fn escape_form_data(s: &str) -> Cow<str> {
    if s.chars().any(|c| c == '"' || c == '\r' || c == '\n') {
        s.replace(['\r', '\n'], "").replace('"', "\\\"").into()
    } else {
        s.into()
    }
}

fn escape_filename(s: &str) -> Cow<str> {
    s.replace('\\', "\\\\")
        .replace('"', "\\\"")
        .into()
}
