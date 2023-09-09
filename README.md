# Tempmail

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
                println!("From: {}", message.from);
                println!("Subject: {}", message.subject);
                println!("Timestamp: {}", message.timestamp);
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
tempmail = "0.2.3"
```

## License

This library is distributed under the terms of the MIT License. See [LICENSE](LICENSE) for details.

## Contributing

Contributions are welcome! Please check out the [contribution guidelines](CONTRIBUTING.md) before getting started.
