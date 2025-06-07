# WhatsApp Handler

[![Crates.io](https://img.shields.io/crates/v/whatsapp_handler.svg)](https://crates.io/crates/whatsapp_handler)
[![Docs.rs](https://docs.rs/whatsapp-handler/badge.svg)](https://docs.rs/whatsapp-handler)
[![License](https://img.shields.io/crates/l/whatsapp_handler.svg)](https://github.com/bambby-plus/whatsapp_handler/blob/main/LICENSE)
[![Repository](https://img.shields.io/badge/github-bambby--plus%2Fwhatsapp__handler-blue?logo=github)](https://github.com/bambby-plus/whatsapp_handler.git)


A Rust library for handling WhatsApp Business API operations, including sending messages and processing incoming webhooks.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
whatsapp_handler = "0.1.0"
tokio = { version = "1.0", features = ["full"] }
```

## Quick Start

### Basic Setup

```rust
use whatsapp_handler::config::Config;

#[tokio::main]
async fn main() {
    let config = Config::from(
        "https://graph.facebook.com".to_string(),    // WhatsApp base URL
        "v17.0".to_string(),                         // API version
        "your_business_id".to_string(),              // WhatsApp Business ID
        "your_phone_number_id".to_string(),          // Phone Number ID
        "your_access_token".to_string(),             // System User Token
    );
}
```

## Sending Messages

### 1. Send Text Message

```rust
use whatsapp_handler::{
    config::Config,
    formatter::outgoing_type::{
        MessageType,
        text::{Content, MType, Text}
    }
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::from(
        "https://graph.facebook.com".to_string(),
        "v17.0".to_string(),
        "your_business_id".to_string(),
        "your_phone_number_id".to_string(),
        "your_access_token".to_string(),
    );

    let message = Text {
        to: "1234567890".to_string(),
        messaging_product: "whatsapp".to_string(),
        recipient_type: "individual".to_string(),
        r#type: MType::text,
        text: Content {
            preview_url: false,
            body: "Hello! This is a message from Rust 🦀".to_string(),
        },
    };

    let response = config.outgoing(MessageType::Text(message)).await?;
    println!("Message sent: {:?}", response);
    
    Ok(())
}
```

### 2. Send Interactive List Message

```rust
use whatsapp_handler::{
    config::Config,
    formatter::outgoing_type::{
        MessageType,
        interactive_list::{
            Action, Body, Footer, Header, InteractiveList, List, MType, Row, Section,
        }
    }
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::from(
        "https://graph.facebook.com".to_string(),
        "v17.0".to_string(),
        "your_business_id".to_string(),
        "your_phone_number_id".to_string(),
        "your_access_token".to_string(),
    );

    let message = InteractiveList {
        to: "1234567890".to_string(),
        messaging_product: "whatsapp".to_string(),
        recipient_type: "individual".to_string(),
        r#type: MType::interactive,
        interactive: List {
            r#type: "list".to_string(),
            header: Header {
                r#type: "text".to_string(),
                text: "Choose a Service".to_string(),
            },
            body: Body {
                text: "Please select one of the following options:".to_string(),
            },
            footer: Footer {
                text: "Powered by Rust".to_string(),
            },
            action: Action {
                button: "View Options".to_string(),
                sections: vec![
                    Section {
                        title: "Services".to_string(),
                        rows: vec![
                            Row {
                                id: "support".to_string(),
                                title: "Customer Support".to_string(),
                                description: "Get help with your account".to_string(),
                            },
                            Row {
                                id: "billing".to_string(),
                                title: "Billing".to_string(),
                                description: "View your billing information".to_string(),
                            },
                        ],
                    },
                    Section {
                        title: "Information".to_string(),
                        rows: vec![
                            Row {
                                id: "about".to_string(),
                                title: "About Us".to_string(),
                                description: "Learn more about our company".to_string(),
                            }
                        ],
                    },
                ],
            },
        },
    };

    let response = config.outgoing(MessageType::InteractiveList(message)).await?;
    println!("Interactive list sent: {:?}", response);
    
    Ok(())
}
```

## Processing Incoming Messages

### Handle Incoming Text Messages

```rust
use whatsapp_handler::config::Config;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::from(
        "https://graph.facebook.com".to_string(),
        "v17.0".to_string(),
        "your_business_id".to_string(),
        "your_phone_number_id".to_string(),
        "your_access_token".to_string(),
    );

    // Example webhook payload from WhatsApp
    let webhook_payload = r#"
    {
        "object": "whatsapp_business_account",
        "entry": [
            {
                "id": "your_business_id",
                "changes": [
                    {
                        "value": {
                            "messaging_product": "whatsapp",
                            "metadata": {
                                "display_phone_number": "1234567890",
                                "phone_number_id": "your_phone_number_id"
                            },
                            "contacts": [
                                {
                                    "profile": {
                                        "name": "John Doe"
                                    },
                                    "wa_id": "1234567890"
                                }
                            ],
                            "messages": [
                                {
                                    "from": "1234567890",
                                    "id": "wamid.unique_id",
                                    "timestamp": "1234567890",
                                    "text": {
                                        "body": "Hello there!"
                                    },
                                    "type": "text"
                                }
                            ]
                        },
                        "field": "messages"
                    }
                ]
            }
        ]
    }
    "#;

    let messages = config.incoming_message(webhook_payload);
    println!("Received messages: {:?}", messages);
    
    Ok(())
}
```

### Handle Message Status Updates

```rust
use whatsapp_handler::config::Config;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::from(
        "https://graph.facebook.com".to_string(),
        "v17.0".to_string(),
        "your_business_id".to_string(),
        "your_phone_number_id".to_string(),
        "your_access_token".to_string(),
    );

    // Example status webhook payload
    let status_payload = r#"
    {
        "object": "whatsapp_business_account",
        "entry": [
            {
                "id": "your_business_id",
                "changes": [
                    {
                        "value": {
                            "messaging_product": "whatsapp",
                            "metadata": {
                                "display_phone_number": "1234567890",
                                "phone_number_id": "your_phone_number_id"
                            },
                            "statuses": [
                                {
                                    "id": "wamid.unique_id",
                                    "status": "delivered",
                                    "timestamp": "1234567890",
                                    "recipient_id": "1234567890"
                                }
                            ]
                        },
                        "field": "messages"
                    }
                ]
            }
        ]
    }
    "#;

    let statuses = config.incoming_statuses(status_payload);
    println!("Message statuses: {:?}", statuses);
    
    Ok(())
}
```

## Complete Example: Echo Bot

```rust
use whatsapp_handler::{
    config::Config,
    formatter::outgoing_type::{
        MessageType,
        text::{Content, MType, Text}
    }
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::from(
        "https://graph.facebook.com".to_string(),
        "v17.0".to_string(),
        std::env::var("WHATSAPP_BUSINESS_ID")?,
        std::env::var("WHATSAPP_PHONE_NUMBER_ID")?,
        std::env::var("WHATSAPP_ACCESS_TOKEN")?,
    );

    // Simulate receiving a message
    let webhook_payload = r#"..."#; // Your webhook payload here
    
    let messages = config.incoming_message(webhook_payload);
    
    // Echo back received messages
    if let Ok(parsed_messages) = messages {
        for message in parsed_messages {
            let echo_message = Text {
                to: message.from, // Send back to sender
                messaging_product: "whatsapp".to_string(),
                recipient_type: "individual".to_string(),
                r#type: MType::text,
                text: Content {
                    preview_url: false,
                    body: format!("Echo: {}", message.text.body),
                },
            };

            let response = config.outgoing(MessageType::Text(echo_message)).await?;
            println!("Echo sent: {:?}", response);
        }
    }
    
    Ok(())
}
```

## Environment Variables

For production usage, store your credentials as environment variables:

```bash
export WHATSAPP_BUSINESS_ID="your_business_id"
export WHATSAPP_PHONE_NUMBER_ID="your_phone_number_id"
export WHATSAPP_ACCESS_TOKEN="your_access_token"
```

## Features

- ✅ Send text messages
- ✅ Send interactive list messages
- ✅ Process incoming messages
- ✅ Handle message status updates
- ✅ Async/await support
- ✅ Type-safe message handling

## License

This project is licensed under the MIT License.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.