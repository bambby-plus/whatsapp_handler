use crate::config::Config;
use crate::formatter::outgoing_type::MessageType;
use reqwest::{Client, header};
use serde_json::Value;

/// Sends a WhatsApp message using the configured API details.
///
/// # Arguments
///
/// * `config` - Reference to the `Config` struct containing WhatsApp API details 
///              like base URL, version, phone number ID, and auth token.
/// * `message` - A reference to the `MessageType` enum, which wraps the outgoing message payload.
///
/// # Returns
///
/// A `Result` with:
/// - `Ok(Value)` → the parsed JSON response from the WhatsApp API if the request was successful.
/// - `Err(reqwest::Error)` → an error if the HTTP request or response parsing failed.
///
/// # Example
///
/// ```ignore
/// let response = send(&config, &message).await?;
/// println!("WhatsApp response: {:?}", response);
/// ```
pub async fn send(config: &Config, message: &MessageType) -> Result<Value, reqwest::Error> {
    let client = Client::new();

    println!("message ====> {:?}", &message);

    let resp = client
        .post(format!(
            "{}/{}/{}/messages",
            config.whatsapp_base_url, config.whatsapp_version, config.whatsapp_phone_number_id
        ))
        .header(header::CONTENT_TYPE, "application/json")
        .header(
            "Authorization",
            format!("Bearer {}", config.whatsapp_system_user_token),
        )
        .json(message)
        .send()
        .await?
        .json::<Value>()
        .await?;

    Ok(resp)
}
