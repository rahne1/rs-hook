# rs-hook

A lightweight, production-ready Discord webhook library for Rust built on `hyper`.

**New to Rust or webhooks?** Start with our [Getting Started Guide](GETTING_STARTED.md) 🚀

## Features

- **Simple & Intuitive API**: Builder patterns for messages and embeds
- **Full Discord Support**: Rich embeds, attachments, allowed mentions, TTS
- **Modular Design**: Easy to integrate into larger codebases
- **Production Ready**: Comprehensive error handling and validation
- **Type Safe**: Leverages Rust's type system for compile-time safety

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
rs-hook = "0.1"
```

## Quick Start

```rust
use rs_hook::{MessageBuilder, Webhook};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let webhook = Webhook::new("https://discord.com/api/webhooks/...")?;

    let message = MessageBuilder::new()
        .content("Hello, Discord!")
        .username("My Bot")
        .build()?;

    webhook.send(message).await?;
    Ok(())
}
```

## Examples

### Sending Embeds

```rust
use rs_hook::{Embed, MessageBuilder, Webhook};

let embed = Embed {
    title: Some("Title".to_string()),
    description: Some("Description".to_string()),
    color: Some(0x5865F2),
    ..Default::default()
};

let message = MessageBuilder::new()
    .content("Check this out!")
    .embed(embed)
    .build()?;

webhook.send(message).await?;
```

### Rich Embeds with Fields

```rust
use rs_hook::{Embed, EmbedAuthor, EmbedField, EmbedFooter, MessageBuilder};

let embed = Embed {
    title: Some("Rich Embed".to_string()),
    author: Some(EmbedAuthor {
        name: "Author".to_string(),
        url: Some("https://example.com".to_string()),
        icon_url: Some("https://...".to_string()),
        ..Default::default()
    }),
    fields: vec![
        EmbedField {
            name: "Field 1".to_string(),
            value: "Value 1".to_string(),
            inline: true,
        },
        EmbedField {
            name: "Field 2".to_string()),
            value: "Value 2".to_string(),
            inline: true,
        },
    ],
    footer: Some(EmbedFooter {
        text: "Footer".to_string(),
        ..Default::default()
    }),
    ..Default::default()
};
```

### Sending Attachments

```rust
use rs_hook::{Attachment, MessageBuilder, Webhook};
use std::path::PathBuf;

let message = MessageBuilder::new()
    .content("Here's a file!")
    .build()?;

let attachment = Attachment {
    path: PathBuf::from("file.txt"),
    description: Some("A text file".to_string()),
};

webhook.send_with_attachments(message, vec![attachment]).await?;
```

### Multiple Embeds

```rust
use rs_hook::{Embed, MessageBuilder};

let embed1 = Embed {
    title: Some("First".to_string()),
    color: Some(0x5865F2),
    ..Default::default()
};

let embed2 = Embed {
    title: Some("Second".to_string()),
    color: Some(0x57F287),
    ..Default::default()
};

let message = MessageBuilder::new()
    .embeds(vec![embed1, embed2])
    .build()?;
```

## API Reference

### Webhook

The main client for sending webhooks.

```rust
let webhook = Webhook::new(url)?;
let webhook = Webhook::new(url)?.with_timeout(30); // 30 second timeout

webhook.send(message).await?;
webhook.send_with_attachments(message, attachments).await?;
```

### MessageBuilder

Builder for Discord webhook messages.

```rust
MessageBuilder::new()
    .content("Message content")
    .username("Override username")
    .avatar_url("https://...")
    .tts(true)
    .embed(embed)
    .embeds(vec![embed1, embed2])
    .allow_mention(AllowedMention::Users)
    .build()?
```

### Embed

Discord rich embed structure.

```rust
Embed {
    title: Some("Title".to_string()),
    description: Some("Description".to_string()),
    url: Some("https://...".to_string()),
    color: Some(0x00FF00),
    timestamp: Some("2024-01-01T00:00:00Z".to_string()),
    footer: Some(EmbedFooter { ... }),
    image: Some(EmbedMedia { ... }),
    thumbnail: Some(EmbedMedia { ... }),
    author: Some(EmbedAuthor { ... }),
    fields: vec![EmbedField { ... }],
    ..Default::default()
}
```

### Error Handling

The library uses a custom `WebhookError` type:

```rust
use rs_hook::{WebhookError, Result};

async fn send_webhook() -> Result<()> {
    webhook.send(message).await?;
    Ok(())
}
```

## Limitations

- Content limited to 6000 characters
- Maximum 10 embeds per message
- Maximum 25 embed fields per embed
- Maximum 10 attachments per message

## License

MIT

## Contributing

Contributions are welcome! Please open an issue or submit a pull request.
