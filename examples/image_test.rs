use rs_hook::{Attachment, Embed, EmbedMedia, MessageBuilder, Webhook};
use std::path::PathBuf;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    let webhook_url = std::env::var("DISCORD_WEBHOOK_URL")
        .expect("DISCORD_WEBHOOK_URL environment variable must be set");

    simple_message_with_attachment(&webhook_url).await?;
    embed_message_with_attachment(&webhook_url).await?;
    embed_message_with_inline_image(&webhook_url).await?;
    embed_message_with_url_image(&webhook_url).await?;

    Ok(())
}

async fn simple_message_with_attachment(
    webhook_url: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("Sending falco.png webhook...");

    let webhook = Webhook::new(webhook_url)?;

    let message = MessageBuilder::new()
        .content("Here's an image of falco!")
        .build()?;

    let attachment = Attachment {
        path: PathBuf::from("examples/falco.png"),
        description: Some("Falco image".to_string()),
    };

    let response = webhook
        .send_with_attachments(message, vec![attachment])
        .await?;

    println!("Webhook sent successfully!");
    println!("Status: {}", response.status_code);
    println!("Body: {}", response.body);

    Ok(())
}

async fn embed_message_with_attachment(
    webhook_url: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("Sending falco.png webhook...");

    let webhook = Webhook::new(webhook_url)?;

    let embed = Embed {
        title: Some("Falco".to_string()),
        description: Some("This is Falco landing into a shine.".to_string()),
        color: Some(0x57F287),
        ..Default::default()
    };

    let message = MessageBuilder::new()
        .content("Here's an image of falco!")
        .embed(embed)
        .build()?;

    let attachment = Attachment {
        path: PathBuf::from("examples/falco.png"),
        description: Some("Falco image".to_string()),
    };

    let response = webhook
        .send_with_attachments(message, vec![attachment])
        .await?;

    println!("Webhook sent successfully!");
    println!("Status: {}", response.status_code);
    println!("Body: {}", response.body);

    Ok(())
}

async fn embed_message_with_inline_image(
    webhook_url: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("Sending embed with inline image...");

    let webhook = Webhook::new(webhook_url)?;

    let attachment = Attachment {
        path: PathBuf::from("examples/falco.png"),
        description: Some("Falco image".to_string()),
    };

    let embed = Embed {
        title: Some("Falco (Inline Image)".to_string()),
        description: Some("This image is embedded in the embed!".to_string()),
        color: Some(0x5865F2),
        image: Some(EmbedMedia {
            url: "attachment://falco.png".to_string(),
            proxy_url: None,
            height: None,
            width: None,
        }),
        ..Default::default()
    };

    let message = MessageBuilder::new()
        .content("Here's an inline embedded image!")
        .embed(embed)
        .build()?;

    let response = webhook
        .send_with_attachments(message, vec![attachment])
        .await?;

    println!("Webhook sent successfully!");
    println!("Status: {}", response.status_code);
    println!("Body: {}", response.body);

    Ok(())
}

async fn embed_message_with_url_image(webhook_url: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("Sending embed with URL image...");

    let webhook = Webhook::new(webhook_url)?;

    let image_url = "https://static.wikia.nocookie.net/smash20xx/images/e/e3/Falco.png/revision/latest/scale-to-width-down/400?cb=20140610000650".to_string();

    let embed = Embed {
        title: Some("Falco (URL Image)".to_string()),
        description: Some("This image uses a URL link!".to_string()),
        color: Some(0xED4245),
        image: Some(EmbedMedia {
            url: image_url,
            proxy_url: None,
            height: None,
            width: None,
        }),
        ..Default::default()
    };

    let message = MessageBuilder::new()
        .content("Here's a URL-based embedded image!")
        .embed(embed)
        .build()?;

    let response = webhook.send(message).await?;

    println!("Webhook sent successfully!");
    println!("Status: {}", response.status_code);
    println!("Body: {}", response.body);

    Ok(())
}
