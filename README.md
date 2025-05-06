# WhatsApp Handler - A Simple Integration for WhatsApp Cloud API Communication

The **WhatsApp Handler** is a comprehensive library designed to simplify communication between your application and the WhatsApp API. With this handler, you can easily manage incoming and outgoing messages, handle multimedia files, and process message statuses, all while maintaining clean and organized code. Built with flexibility in mind, it supports multiple message formats, including text, images, videos, documents, audio, and interactive messages like buttons and lists.

## Key Features:
- **Configuration Management**: Easily manage WhatsApp API credentials and configuration details (e.g., base URL, business ID, phone number ID, and system user token).
- **Incoming Message Handling**: Seamlessly process incoming messages from WhatsApp using webhooks, including structured payloads for text and media content.
- **Interactive Messaging**: Send rich, interactive messages with buttons and lists, providing a dynamic and engaging user experience.
- **Outgoing Message Support**: Efficiently send various message types, such as text, images, videos, audio, stickers, and documents, to recipients through the WhatsApp API.
- **Status Updates**: Track and process message status updates, ensuring you have full visibility of message deliveries, read receipts, and more.
- **Data Safety**: Supports robust error handling and validation, ensuring safe communication and message integrity.

## Modules & Components:
1. **Config Struct**: Manages WhatsApp API configuration details, including the base URL, version, business ID, and system token.
2. **Message Types**: Handles multiple message formats like text, image, video, audio, document, and interactive types (buttons and lists).
3. **Status Handling**: Offers a structured way to manage and process incoming status updates for messages, including delivery and read statuses.
4. **Interactive Elements**: Integrates interactive message formats, such as buttons and lists, for better user engagement.

## Usage Example:
Here’s how you can use the `WhatsApp Handler` to send a text message:

```rust
let config = Config::from(
    "https://graph.facebook.com".to_string(),
    "v16.0".to_string(),
    "your-business-id".to_string(),
    "your-phone-id".to_string(),
    "your-token".to_string(),
);

let message = MessageType::Text(text::Text {
    to: "recipient-id".to_string(),
    messaging_product: "whatsapp".to_string(),
    recipient_type: "individual".to_string(),
    r#type: text::MType::text,
    text: text::Content {
        preview_url: false,
        body: "Hello, World!".to_string(),
    },
});

let response = config.outgoing(message).await;


```



The following example demonstrates how to process incoming WhatsApp messages from webhooks using the WhatsApp handler. The payload is received from WhatsApp, and the code processes it to extract relevant message information.

### Code:

```rust
let config = Config::from(
    "https://graph.facebook.com".to_string(),
    "v16.0".to_string(),
    "your-business-id".to_string(),
    "your-phone-id".to_string(),
    "your-token".to_string(),
);

let webhook_payload = r#"{
    "object": "whatsapp",
    "entry": [
        {
            "id": "your-entry-id",
            "changes": [
                {
                    "value": {
                        "messaging_product": "whatsapp",
                        "contacts": [
                            {
                                "profile": {"name": "John Doe"},
                                "wa_id": "recipient-id"
                            }
                        ],
                        "messages": [
                            {
                                "from": "recipient-id",
                                "id": "message-id",
                                "timestamp": "timestamp",
                                "text": {
                                    "body": "Hello, this is a test message."
                                }
                            }
                        ]
                    },
                    "field": "messages"
                }
            ]
        }
    ]
}"#;

match config.incoming_message(webhook_payload) {
    Ok((messages, _)) => {
        for message in messages {
            // Process each message as needed
            println!("Incoming message: {:?}", message);
        }
    }
    Err(e) => {
        eprintln!("Error processing incoming message: {}", e);
    }
}

```
