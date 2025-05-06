use crate::action::incoming;
use crate::action::outgoing;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::formatter::outgoing_type::MessageType;

/// Configuration details for communicating with the WhatsApp API.
#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    /// Base URL of the WhatsApp API (e.g., https://graph.facebook.com).
    pub whatsapp_base_url: String,

    /// WhatsApp API version (e.g., v17.0).
    pub whatsapp_version: String,

    /// WhatsApp Business Account ID.
    pub whatsapp_business_id: String,

    /// WhatsApp Phone Number ID.
    pub whatsapp_phone_number_id: String,

    /// System user access token for API authorization.
    pub whatsapp_system_user_token: String,
}

impl Config {
    /// Creates a new `Config` instance.
    ///
    /// # Arguments
    /// - `whatsapp_base_url`: Base API URL.
    /// - `whatsapp_version`: API version.
    /// - `whatsapp_business_id`: Business account ID.
    /// - `whatsapp_phone_number_id`: Phone number ID.
    /// - `whatsapp_system_user_token`: Authorization token.
    ///
    /// # Example
    /// ```
    /// let config = config::Config::from(
    ///     "https://graph.facebook.com".to_string(),
    ///     "v17.0".to_string(),
    ///     "your-business-id".to_string(),
    ///     "your-phone-id".to_string(),
    ///     "your-token".to_string(),
    /// );
    /// ```
    pub fn from(
        whatsapp_base_url: String,
        whatsapp_version: String,
        whatsapp_business_id: String,
        whatsapp_phone_number_id: String,
        whatsapp_system_user_token: String,
    ) -> Self {
        Config {
            whatsapp_base_url,
            whatsapp_version,
            whatsapp_business_id,
            whatsapp_phone_number_id,
            whatsapp_system_user_token,
        }
    }

    /// Processes incoming WhatsApp message payloads (typically from webhooks).
    ///
    /// # Arguments
    /// - `payload`: Raw JSON string received from the WhatsApp webhook.
    ///
    /// # Returns
    /// A `Result` with a tuple containing:
    /// - `Vec<Value>`: List of parsed message objects.
    /// - `Vec<String>`: List of errors associated each metadata or extracted details.
    ///
    /// Returns `Err(String)` if parsing fails.
    pub fn incoming_message(self, payload: &str) -> Result<(Vec<Value>, Vec<String>), String> {
        incoming::find_messages(self, payload)
    }

    /// Processes incoming WhatsApp status updates (typically from webhooks).
    ///
    /// # Arguments
    /// - `payload`: Raw JSON string received from the WhatsApp webhook.
    ///
    /// # Returns
    /// A `Result` with a tuple containing:
    /// - `Vec<Value>`: List of parsed status objects.
    /// - `Vec<String>`: List of errors associated each metadata or extracted details.
    ///
    /// Returns `Err(String)` if parsing fails.
    pub fn incoming_statuses(self, payload: &str) -> Result<(Vec<Value>, Vec<String>), String> {
        incoming::find_statuses(self, payload)
    }

    /// Sends an outgoing WhatsApp message using the configured API details.
    ///
    /// # Arguments
    /// - `message`: A structured `MessageType` representing the message to send.
    ///
    /// # Returns
    /// A `Result` with:
    /// - `Ok(Value)`: The API response as a JSON value.
    /// - `Err(reqwest::Error)`: If the HTTP request fails.
    ///
    /// # Example
    /// ```
    /// let response = config.outgoing(message).await?;
    /// ```
    pub async fn outgoing(self, message: MessageType) -> Result<Value, reqwest::Error> {
        outgoing::send(&self, &message).await
    }
}
