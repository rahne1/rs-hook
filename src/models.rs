use serde::Serialize;
use std::path::PathBuf;

fn is_false(b: &bool) -> bool {
    !b
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct MessageBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar_url: Option<String>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub embeds: Vec<Embed>,

    #[serde(skip_serializing_if = "crate::models::is_false")]
    pub tts: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_mentions: Option<AllowedMentions>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AllowedMention {
    Users,
    Roles,
    Everyone,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct AllowedMentions {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub parse: Vec<AllowedMention>,
}

impl MessageBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn content(mut self, content: impl Into<String>) -> Self {
        self.content = Some(content.into());
        self
    }

    pub fn username(mut self, username: impl Into<String>) -> Self {
        self.username = Some(username.into());
        self
    }

    pub fn avatar_url(mut self, url: impl Into<String>) -> Self {
        self.avatar_url = Some(url.into());
        self
    }

    pub fn embed(mut self, embed: Embed) -> Self {
        self.embeds.push(embed);
        self
    }

    pub fn embeds(mut self, embeds: Vec<Embed>) -> Self {
        self.embeds.extend(embeds);
        self
    }

    pub fn tts(mut self, tts: bool) -> Self {
        self.tts = tts;
        self
    }

    pub fn allow_mention(mut self, mention: AllowedMention) -> Self {
        if self.allowed_mentions.is_none() {
            self.allowed_mentions = Some(AllowedMentions::default());
        }
        self.allowed_mentions.as_mut().unwrap().parse.push(mention);
        self
    }

    pub fn build(self) -> crate::error::Result<Self> {
        if let Some(ref content) = self.content {
            if content.len() > 6000 {
                return Err(crate::error::WebhookError::ContentTooLong(content.len()));
            }
        }
        if self.embeds.len() > 10 {
            return Err(crate::error::WebhookError::Request("Too many embeds (max 10)".to_string()));
        }
        Ok(self)
    }
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct Embed {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub footer: Option<EmbedFooter>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<EmbedMedia>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<EmbedMedia>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub video: Option<EmbedMedia>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider: Option<EmbedProvider>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<EmbedAuthor>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub fields: Vec<EmbedField>,
}

#[derive(Debug, Clone, Serialize)]
pub struct EmbedFooter {
    pub text: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_icon_url: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct EmbedMedia {
    pub url: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<u64>,
}

#[derive(Debug, Clone, Serialize)]
pub struct EmbedProvider {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct EmbedAuthor {
    pub name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_icon_url: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct EmbedField {
    pub name: String,
    pub value: String,

    #[serde(skip_serializing_if = "is_false")]
    #[serde(default)]
    pub inline: bool,
}

#[derive(Debug, Clone)]
pub struct Attachment {
    pub path: PathBuf,
    pub description: Option<String>,
}

#[derive(Debug, Clone)]
pub struct WebhookResponse {
    pub status_code: u16,
    pub body: String,
}
