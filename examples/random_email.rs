use tempmail::Tempmail;

#[tokio::main]
async fn main() {
    // Create a new Tempmail instance with a random username and domain
    let tempmail = Tempmail::random();

    println!(
        "Got a random temporary email: {}@{}",
        tempmail.username, tempmail.domain
    );

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
