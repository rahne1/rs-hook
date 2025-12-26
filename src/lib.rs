mod error;
mod models;
mod multipart;
mod webhook;

pub use error::{WebhookError, Result};
pub use models::{
    AllowedMention, AllowedMentions, Attachment, Embed, EmbedAuthor, EmbedField, EmbedFooter, EmbedMedia, EmbedProvider,
    MessageBuilder, WebhookResponse,
};
pub use webhook::Webhook;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_message_builder() {
        let message = MessageBuilder::new()
            .content("Test message")
            .username("Test Bot")
            .tts(false)
            .build();

        assert!(message.is_ok());
    }

    #[test]
    fn test_embed_builder() {
        let embed = Embed {
            title: Some("Test Title".to_string()),
            description: Some("Test Description".to_string()),
            color: Some(0x00ff00),
            ..Default::default()
        };

        assert_eq!(embed.title, Some("Test Title".to_string()));
        assert_eq!(embed.color, Some(0x00ff00));
    }

    #[test]
    fn test_content_length_validation() {
        let long_content = "x".repeat(6001);
        let message = MessageBuilder::new()
            .content(long_content)
            .build();

        assert!(message.is_err());
    }

    #[test]
    fn test_embed_count_validation() {
        let embeds: Vec<Embed> = (0..11).map(|_| Embed::default()).collect();
        let message = MessageBuilder::new()
            .content("Test")
            .embeds(embeds)
            .build();

        assert!(message.is_err());
    }
}
