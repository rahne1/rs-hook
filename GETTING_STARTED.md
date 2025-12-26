# Getting Started with rs-hook

A beginner's guide to sending Discord webhooks using Rust.

## What is rs-hook?

`rs-hook` is a Rust library that lets you send messages to Discord channels using webhooks.

## Prerequisites

- Rust installed (if not, visit https://rustup.rs)
- Basic familiarity with Rust
- A Discord webhook URL

## Step 1: Get a Discord Webhook URL

1. Open your Discord server
2. Go to **Server Settings** → **Integrations** → **Webhooks**
3. Click **New Webhook**
4. Name it (e.g., "My Bot")
5. Copy the webhook URL (it looks like: `https://discord.com/api/webhooks/12345/abcde...`)

## Step 2: Create a New Rust Project

```bash
cargo new my-discord-bot
cd my-discord-bot
```

## Step 3: Add rs-hook to Your Project

Open `Cargo.toml` and add:

```toml
[dependencies]
rs-hook = "0.1"
tokio = { version = "1", features = ["full"] }
```

## Step 4: Write Your First Webhook

Open `src/main.rs` and replace with:

```rust
use rs_hook::{MessageBuilder, Webhook};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let webhook_url = "YOUR_WEBHOOK_URL_HERE";
    
    let webhook = Webhook::new(webhook_url)?;
    
    let message = MessageBuilder::new()
        .content("Hello from Rust! 🦀")
        .build()?;
    
    webhook.send(message).await?;
    
    println!("Message sent!");
    Ok(())
}
```

Replace `YOUR_WEBHOOK_URL_HERE` with your actual webhook URL.

## Step 5: Run It

```bash
cargo run
```

Check your Discord channel - you should see your message!

---

## Common Use Cases

### Sending a Simple Message

```rust
let message = MessageBuilder::new()
    .content("Hello, world!")
    .build()?;

webhook.send(message).await?;
```

### Using a Custom Username

```rust
let message = MessageBuilder::new()
    .content("Message")
    .username("Cool Bot")
    .build()?;
```

### Sending an Embed (Rich Message)

```rust
use rs_hook::Embed;

let embed = Embed {
    title: Some("Important Update".to_string()),
    description: Some("Here's some information".to_string()),
    color: Some(0x00ff00), // Green color
    ..Default::default()
};

let message = MessageBuilder::new()
    .content("Check out this embed!")
    .embed(embed)
    .build()?;

webhook.send(message).await?;
```

### Sending a File Attachment

```rust
use rs_hook::Attachment;
use std::path::PathBuf;

let message = MessageBuilder::new()
    .content("Here's a file!")
    .build()?;

let attachment = Attachment {
    path: PathBuf::from("/path/to/your/file.txt"),
    description: Some("My file".to_string()),
};

webhook.send_with_attachments(message, vec![attachment]).await?;
```

### Sending an Image

```rust
let message = MessageBuilder::new()
    .content("Here's an image!")
    .build()?;

let attachment = Attachment {
    path: PathBuf::from("/home/username/image.png"),
    description: Some("A cool image".to_string()),
};

webhook.send_with_attachments(message, vec![attachment]).await?;
```

### Rich Embed with Fields

```rust
use rs_hook::{Embed, EmbedField, EmbedFooter};

let embed = Embed {
    title: Some("Server Status".to_string()),
    fields: vec![
        EmbedField {
            name: "CPU".to_string(),
            value: "45%".to_string(),
            inline: true,
        },
        EmbedField {
            name: "Memory".to_string(),
            value: "2.1 GB".to_string(),
            inline: true,
        },
    ],
    footer: Some(EmbedFooter {
        text: "Updated just now".to_string(),
        ..Default::default()
    }),
    ..Default::default()
};

let message = MessageBuilder::new()
    .content("Server update")
    .embed(embed)
    .build()?;
```

---

## Using Environment Variables (Best Practice)

Never hardcode your webhook URL! Use environment variables instead:

```bash
# In your terminal
export DISCORD_WEBHOOK_URL="https://discord.com/api/webhooks/..."

# Run your app
cargo run
```

In your code:

```rust
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let webhook_url = env::var("DISCORD_WEBHOOK_URL")
        .expect("DISCORD_WEBHOOK_URL not set");
    
    let webhook = Webhook::new(webhook_url)?;
    // ... rest of your code
}
```

---

## Error Handling

rs-hook provides helpful error messages:

```rust
use rs_hook::{WebhookError, Result};

async fn send_message() -> Result<()> {
    webhook.send(message).await?;
    Ok(())
}
```

Common errors:
- `InvalidUrl` - Your webhook URL is wrong
- `ContentTooLong` - Message exceeds 6000 characters
- `Status { status, body }` - Discord rejected the request (check `body` for details)

---

## Tips & Tricks

### Color Codes

Embed colors are hexadecimal:
- Red: `0xff0000`
- Green: `0x00ff00`
- Blue: `0x0000ff`
- Purple: `0x5865F2` (Discord blurple)
- Orange: `0x00D26A` (Discord green)

### Multiple Embeds

```rust
let embed1 = Embed { /* ... */ };
let embed2 = Embed { /* ... */ };

let message = MessageBuilder::new()
    .embeds(vec![embed1, embed2])
    .build()?;
```

### Timeout

Set a timeout for slow connections:

```rust
let webhook = Webhook::new(url)?
    .with_timeout(10); // 10 seconds
```

---

## Troubleshooting

### "Invalid URL" Error

Make sure your URL contains `discord.com/api/webhooks/` or `discordapp.com/api/webhooks/`

### Nothing Shows Up in Discord

- Check you have the correct webhook URL
- Make sure the webhook is active in your Discord server settings
- Check the console for error messages

### "Content Too Long" Error

Discord limits messages to 6000 characters. Shorten your message or use embeds.

### Permission Denied on File

Make sure Rust has permission to read the file:
```bash
chmod 644 /path/to/your/file
```

---

## Complete Example

Here's a complete example that sends a rich embed with an image:

```rust
use rs_hook::{Attachment, Embed, EmbedAuthor, EmbedField, EmbedFooter, MessageBuilder, Webhook};
use std::env;
use std::path::PathBuf;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let webhook_url = env::var("DISCORD_WEBHOOK_URL")?;
    
    let webhook = Webhook::new(&webhook_url)?;
    
    let embed = Embed {
        title: Some("Project Update".to_string()),
        description: Some("We made great progress today!".to_string()),
        color: Some(0x5865F2),
        author: Some(EmbedAuthor {
            name: "Dev Team".to_string(),
            icon_url: Some("https://cdn.discordapp.com/embed/avatars/0.png".to_string()),
            ..Default::default()
        }),
        fields: vec![
            EmbedField {
                name: "Tasks Completed".to_string(),
                value: "5".to_string(),
                inline: true,
            },
            EmbedField {
                name: "Bugs Fixed".to_string(),
                value: "3".to_string(),
                inline: true,
            },
        ],
        footer: Some(EmbedFooter {
            text: "Daily Report".to_string(),
            ..Default::default()
        }),
        ..Default::default()
    };
    
    let message = MessageBuilder::new()
        .content("📊 Daily update")
        .username("Project Bot")
        .embed(embed)
        .build()?;
    
    webhook.send(message).await?;
    
    println!("Report sent successfully!");
    Ok(())
}
```

---

## Next Steps

- Check the [README.md](README.md) for full API documentation
- Look at [examples/basic_usage.rs](examples/basic_usage.rs) for more examples
- Explore the source code to understand how it works

## Need Help?

- Open an issue on GitHub
- Check Discord's official webhook documentation: https://discord.com/developers/docs/resources/webhook

Happy coding! 🦀
