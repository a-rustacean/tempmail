use tempmail::{Domain, Tempmail};

#[tokio::main]
async fn main() {
    // Create a new Tempmail instance with a custom username and domain
    let tempmail =
        Tempmail::new("customEmailAddress", Some(Domain::SecMailNet));

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
