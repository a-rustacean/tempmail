//! # Tempmail
//!
//! A Rust library for interacting with temporary email services.
//!
//! This library provides functionality to manage temporary email addresses,
//! retrieve messages, and download attachments using various temporary email
//! domains.
//!
//! # Example
//!
//! ```rust
//! use tempmail::{Domain, Tempmail};
//!
//! #[tokio::main]
//! async fn main() {
//!     // Create a new Tempmail instance with a custom username and domain.
//!     let tempmail = Tempmail::new("example", Some(Domain::SecMailOrg));
//!
//!     // Get a list of messages from the temporary email inbox.
//!     let messages = tempmail.get_messages().await;
//!
//!     match messages {
//!         Ok(messages) => {
//!             for message in messages {
//!                 println!("From: {}", message.from);
//!                 println!("Subject: {}", message.subject);
//!                 println!("Timestamp: {}", message.timestamp);
//!             }
//!         }
//!         Err(error) => {
//!             eprintln!("Error: {}", error);
//!         }
//!     }
//! }
//! ```
//!
//! The Tempmail library allows you to create a temporary email address,
//! retrieve messages from the inbox, and download attachments associated with
//! messages.
//!
//! The library defines several data structures, enums, and methods to
//! facilitate these interactions. Refer to the documentation for individual
//! struct and enum definitions for more details.

use std::fmt::Display;

use chrono::prelude::*;
#[cfg(target_arch = "wasm32")]
use js_sys::Math;
#[cfg(not(target_arch = "wasm32"))]
use rand::{rng, Rng};
use serde::{Deserialize, Deserializer};

const API_URL: &str = "https://www.1secmail.com/api/v1/";

/// Represents an attachment associated with an email message.
#[derive(Debug, Clone, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct Attachment {
    /// The filename of the attachment.
    pub filename: String,
    /// The MIME content type of the attachment.
    #[serde(rename = "contentType")]
    pub content_type: String,
    /// The size of the attachment in bytes.
    pub size: usize,
}

/// Represents an email message received in the temporary email inbox.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Message {
    /// The unique identifier of the message.
    pub id: usize,
    /// The sender's email address.
    pub from: String,
    /// The subject of the email.
    pub subject: String,
    /// The timestamp when the email was received.
    pub timestamp: DateTime<Utc>,
    /// A vector of `TempmailAttachment` representing attachments in the email.
    pub attachments: Vec<Attachment>,
    /// The full body of the email, including both text and HTML content.
    pub body: String,
    /// The text-only content of the email body.
    pub text_body: String,
    /// Optional HTML content of the email body.
    pub html_body: Option<String>,
}

/// Represents a raw version of an email message with minimal information.
#[derive(Debug, Clone)]
pub struct RawMessage {
    /// The unique identifier of the message.
    pub id: usize,
    /// The sender's email address
    pub from: String,
    /// The subject of the mail
    pub subject: String,
    /// The timestamp when the email was received
    pub timestamp: DateTime<Utc>,
}

/// Enum representing different temporary email domains.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Domain {
    /// Domain "1secmail.com"
    SecMailCom,
    /// Domain "1secmail.org"
    SecMailOrg,
    /// Domain "1secmail.net"
    SecMailNet,
    /// Domain "wwjmp.com"
    WwjmpCom,
    /// Domain "esiix.com"
    EsiixCom,
    /// Domain "xojxe.com"
    XojxeCom,
    /// Domain "yoggm.com"
    YoggmCom,
}

/// Represents a temporary email address with associated domain for receiving
/// emails.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Tempmail {
    /// The username part of the email address.
    pub username: String,
    /// The selected domain for the temporary email.
    pub domain: Domain,
}

pub type TempmailError = reqwest::Error;

/// A type alias for `Result` with `TempmailError` as the error type.
pub type TempmailResult<T> = Result<T, TempmailError>;

#[derive(Deserialize)]
struct MessageWrapper {
    id: usize,
    from: String,
    subject: String,
    #[serde(rename = "date")]
    timestamp: String,
    attachments: Vec<Attachment>,
    body: String,
    #[serde(rename = "textBody")]
    text_body: String,
    #[serde(rename = "htmlBody")]
    html_body: Option<String>,
}

#[derive(Deserialize)]
struct RawMessageWrapper {
    id: usize,
    from: String,
    subject: String,
    #[serde(rename = "date")]
    timestamp: String,
}

impl<'de> Deserialize<'de> for Message {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let wrapper: MessageWrapper = Deserialize::deserialize(deserializer)?;

        let timestamp = NaiveDateTime::parse_from_str(
            &wrapper.timestamp,
            "%Y-%m-%d %H:%M:%S",
        )
        .map(|ndt| Utc.from_utc_datetime(&ndt))
        .map_err(serde::de::Error::custom)?;

        Ok(Message {
            id: wrapper.id,
            from: wrapper.from,
            subject: wrapper.subject,
            timestamp,
            attachments: wrapper.attachments,
            body: wrapper.body,
            text_body: wrapper.text_body,
            html_body: wrapper.html_body,
        })
    }
}

impl<'de> Deserialize<'de> for RawMessage {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let wrapper: RawMessageWrapper =
            Deserialize::deserialize(deserializer)?;

        let timestamp = NaiveDateTime::parse_from_str(
            &wrapper.timestamp,
            "%Y-%m-%d %H:%M:%S",
        )
        .map(|ndt| Utc.from_utc_datetime(&ndt))
        .map_err(serde::de::Error::custom)?;

        Ok(RawMessage {
            id: wrapper.id,
            from: wrapper.from,
            subject: wrapper.subject,
            timestamp,
        })
    }
}
impl Domain {
    const DOMAINS: [Domain; 7] = [
        Domain::SecMailCom,
        Domain::SecMailOrg,
        Domain::SecMailNet,
        Domain::WwjmpCom,
        Domain::EsiixCom,
        Domain::XojxeCom,
        Domain::YoggmCom,
    ];

    pub fn random() -> Self {
        let index = (random() * Self::DOMAINS.len() as f64).round() as usize;
        Self::DOMAINS[index].clone()
    }
}

impl Display for Domain {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Domain::SecMailCom => f.write_str("1secmail.com"),
            Domain::SecMailOrg => f.write_str("1secmail.org"),
            Domain::SecMailNet => f.write_str("1secmail.net"),
            Domain::WwjmpCom => f.write_str("wwjmp.com"),
            Domain::EsiixCom => f.write_str("esiix.com"),
            Domain::XojxeCom => f.write_str("xojxe.com"),
            Domain::YoggmCom => f.write_str("yoggm.com"),
        }
    }
}

impl Default for Domain {
    fn default() -> Self {
        Self::SecMailCom
    }
}

// A helper function to perform a JSON GET request and deserialize the response.
async fn reqjson<T, R>(query: T) -> TempmailResult<R>
where
    T: AsRef<str>,
    R: for<'de> Deserialize<'de>,
{
    reqwest::get(format!("{}?{}", API_URL, query.as_ref())).await?.json().await
}

#[cfg(not(target_arch = "wasm32"))]
fn random() -> f64 {
    let mut rng = rng();
    rng.random_range(0.0..1.0)
}

#[cfg(target_arch = "wasm32")]
fn random() -> f64 {
    Math::random()
}

fn generate_random_string(length: usize) -> String {
    let mut random_string = String::with_capacity(length);

    let characters: Vec<char> =
        "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789"
            .chars()
            .collect();

    for _ in 0..length {
        let random_index = (random() * characters.len() as f64) as usize;
        random_string.push(characters[random_index]);
    }

    random_string
}

impl Tempmail {
    /// Creates a new instance of the Tempmail struct.
    pub fn new<U>(username: U, domain: Option<Domain>) -> Self
    where
        U: Into<String>,
    {
        Self { username: username.into(), domain: domain.unwrap_or_default() }
    }

    /// Creates a new instance of the Tempmail struct with random username and
    /// domain
    pub fn random() -> Self {
        let len = (10.0 + random() * 40.0).floor() as usize;
        let username = generate_random_string(len);
        let domain = Domain::random();

        Self { username, domain }
    }

    /// Fetches the messages in the inbox.
    pub async fn get_messages(&self) -> TempmailResult<Vec<Message>> {
        let raw_messages: Vec<RawMessage> = reqjson(format!(
            "action=getMessages&login={}&domain={}",
            self.username, self.domain
        ))
        .await?;

        let mut messages = Vec::new();

        for raw_message in raw_messages {
            let mut message: Message = reqjson(format!(
                "action=readMessage&login={}&domain={}&id={}",
                self.username, self.domain, raw_message.id
            ))
            .await?;

            if let Some(html_body) = message.html_body.clone() {
                if html_body.is_empty() {
                    message.html_body = None;
                }
            }

            messages.push(message);
        }

        Ok(messages)
    }

    pub async fn get_raw_messages(&self) -> TempmailResult<Vec<RawMessage>> {
        reqjson(format!(
            "action=getMessages&login={}&domain={}",
            self.username, self.domain
        ))
        .await
    }

    pub async fn expand_raw_message(
        &self,
        raw_message: &RawMessage,
    ) -> TempmailResult<Message> {
        let mut message: Message = reqjson(format!(
            "action=readMessage&login={}&domain={}&id={}",
            self.username, self.domain, raw_message.id
        ))
        .await?;

        if let Some(html_body) = message.html_body.clone() {
            if html_body.is_empty() {
                message.html_body = None;
            }
        }

        Ok(message)
    }

    /// Fetches the attachment of the specified message_id and filename.
    pub async fn get_attachment<T>(
        &self,
        message_id: usize,
        filename: T,
    ) -> TempmailResult<Vec<u8>>
    where
        T: AsRef<str>,
    {
        reqwest::get(format!(
            "{}?action=download&login={}&domain={}&id={}&file={}",
            API_URL,
            self.username,
            self.domain,
            message_id,
            filename.as_ref()
        ))
        .await?
        .bytes()
        .await
        .map(|b| b.to_vec())
    }
}

// `Send` and `Sync` trait implementations for public structs

unsafe impl Send for Domain {}
unsafe impl Sync for Domain {}
unsafe impl Send for Message {}
unsafe impl Sync for Message {}
unsafe impl Send for Tempmail {}
unsafe impl Sync for Tempmail {}
unsafe impl Send for Attachment {}
unsafe impl Sync for Attachment {}
unsafe impl Send for RawMessage {}
unsafe impl Sync for RawMessage {}
