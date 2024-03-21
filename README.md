# Tempmail

[![Version](https://img.shields.io/crates/v/tempmail)](https://crates.io/crates/tempmail)
[![Downloads](https://img.shields.io/crates/d/tempmail)](https://crates.io/crates/tempmail)
[![License](https://img.shields.io/crates/l/tempmail)](https://crates.io/crates/tempmail)
[![Docs](https://docs.rs/tempmail/badge.svg)](https://docs.rs/tempmail)

The Tempmail simplifies temporary email management and interaction, including message retrieval and attachment downloads, using the Rust programming language.

## Features

- Create and manage temporary email addresses.
- Retrieve email messages from the temporary email inbox.
- Download attachments associated with email messages.
- Built-in support for popular temporary email domains.
- Convenient error handling and result types.

## Usage

```rust
use tempmail::{Domain, Tempmail};

#[tokio::main]
async fn main() {
    // Create a new Tempmail instance with a custom username and domain.
    let tempmail = Tempmail::new("example", Some(Domain::SecMailOrg));

    // Get a list of messages from the temporary email inbox.
    let messages = tempmail.get_messages().await;

    match messages {
        Ok(messages) => {
            for message in messages {
                println!("Id: {}", message.id);
                println!("From: {}", message.from);
                println!("Subject: {}", message.subject);
                println!("Timestamp: {}", message.timestamp);
                println!("Attachments:");
                for attachment in message.attachments {
                    println!("  Filename: {}", attachment.filename);
                    println!("  ContentType: {}", attachment.content_type);
                    println!("  Size: {}", attachment.size);
                }
                println!("Body: {}", message.body);
            }
        }
        Err(error) => {
            eprintln!("Error: {}", error);
        }
    }
}
```

## Installation

To use this library in your project, simply add the following to your `Cargo.toml`:

```toml
[dependencies]
tempmail = "1.0.0"
```

## License

This library is distributed under the terms of the MIT License. See [LICENSE](LICENSE) for details.

## Contributing

Contributions are welcome! Please check out the [contribution guidelines](CONTRIBUTING.md) before getting started.
