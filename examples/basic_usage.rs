use rs_hook::{
    AllowedMention, Attachment, Embed, EmbedAuthor, EmbedField, EmbedFooter, EmbedMedia,
    MessageBuilder, Webhook,
};
use std::path::PathBuf;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();

    let webhook_url = std::env::var("DISCORD_WEBHOOK_URL")
        .expect("DISCORD_WEBHOOK_URL environment variable must be set");

    simple_message(&webhook_url).await?;
    embed_message(&webhook_url).await?;
    rich_embed(&webhook_url).await?;
    multiple_embeds(&webhook_url).await?;
    with_attachment(&webhook_url).await?;

    Ok(())
}

async fn simple_message(webhook_url: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("Sending simple message...");

    let webhook = Webhook::new(webhook_url)?;

    let message = MessageBuilder::new().content("Hello, Discord!").build()?;

    let response = webhook.send(message).await?;

    println!("Response: {}", response.status_code);
    Ok(())
}

async fn embed_message(webhook_url: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("Sending message with embed...");

    let webhook = Webhook::new(webhook_url)?;

    let embed = Embed {
        title: Some("Embed Title".to_string()),
        description: Some("This is a simple embed description".to_string()),
        color: Some(0x5865F2),
        ..Default::default()
    };

    let message = MessageBuilder::new()
        .content("Check out this embed!")
        .embed(embed)
        .build()?;

    let response = webhook.send(message).await?;

    println!("Response: {}", response.status_code);
    Ok(())
}

async fn rich_embed(webhook_url: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("Sending rich embed...");

    let webhook = Webhook::new(webhook_url)?;

    let embed = Embed {
        title: Some("Rich Embed Example".to_string()),
        description: Some("This is a comprehensive embed with all the features".to_string()),
        url: Some("https://example.com".to_string()),
        color: Some(0x00D26A),
        author: Some(EmbedAuthor {
            name: "Author Name".to_string(),
            url: Some("https://example.com".to_string()),
            icon_url: Some("https://cdn.discordapp.com/embed/avatars/0.png".to_string()),
            proxy_icon_url: None,
        }),
        fields: vec![
            EmbedField {
                name: "Field 1".to_string(),
                value: "Value 1".to_string(),
                inline: true,
            },
            EmbedField {
                name: "Field 2".to_string(),
                value: "Value 2".to_string(),
                inline: true,
            },
            EmbedField {
                name: "Field 3".to_string(),
                value: "Value 3".to_string(),
                inline: false,
            },
        ],
        footer: Some(EmbedFooter {
            text: "Footer text".to_string(),
            icon_url: Some("https://cdn.discordapp.com/embed/avatars/0.png".to_string()),
            proxy_icon_url: None,
        }),
        image: Some(EmbedMedia {
            url: "https://example.com/image.png".to_string(),
            proxy_url: None,
            height: None,
            width: None,
        }),
        thumbnail: Some(EmbedMedia {
            url: "https://example.com/thumb.png".to_string(),
            proxy_url: None,
            height: None,
            width: None,
        }),
        ..Default::default()
    };

    let message = MessageBuilder::new()
        .content("Here's a rich embed!")
        .embed(embed)
        .allow_mention(AllowedMention::Users)
        .build()?;

    let response = webhook.send(message).await?;

    println!("Response: {}", response.status_code);
    Ok(())
}

async fn multiple_embeds(webhook_url: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("Sending multiple embeds...");

    let webhook = Webhook::new(webhook_url)?;

    let embed1 = Embed {
        title: Some("First Embed".to_string()),
        description: Some("First embed description".to_string()),
        color: Some(0x5865F2),
        ..Default::default()
    };

    let embed2 = Embed {
        title: Some("Second Embed".to_string()),
        description: Some("Second embed description".to_string()),
        color: Some(0x57F287),
        ..Default::default()
    };

    let message = MessageBuilder::new()
        .content("Multiple embeds example")
        .embeds(vec![embed1, embed2])
        .build()?;

    let response = webhook.send(message).await?;

    println!("Response: {}", response.status_code);
    Ok(())
}

async fn with_attachment(webhook_url: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("Sending message with attachment...");

    let webhook = Webhook::new(webhook_url)?;

    let message = MessageBuilder::new()
        .content("Here's a file attachment!")
        .build()?;

    let attachment = Attachment {
        path: PathBuf::from("falco.png"),
        description: Some("Chess game file".to_string()),
    };

    let response = webhook
        .send_with_attachments(message, vec![attachment])
        .await?;

    println!("Response: {}", response.status_code);
    Ok(())
}
