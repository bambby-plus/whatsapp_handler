use crate::config::Config;
use crate::formatter::incoming_type::{
    audio::{Audio, AudioData},
    button::{Button, ButtonDetails, MsgBtnContext},
    contact::{Address, Contact, Email, Name, Organization, Phone, Url},
    document::{Document, DocumentData},
    full_messages_payload::WebhookMessagePayload,
    full_statuses_payload::WebhookStatusesPayload,
    image::{Image, ImageData},
    location::{Location, LocationPayload},
    order::{MessageContext, Order, OrderDetails, ProductItem},
    reaction::{Reaction, ReactionPayload},
    sticker::{Sticker, StickerData},
    text::{Text, TextPayload},
    unknown::{ErrorDetail, Unknown},
    video::{Video, VideoData},
};
use serde_json::{Value, json};

pub fn find_messages(config: Config, message: &str) -> Result<(Vec<Value>, Vec<String>), String> {
    let mut error_resp: Vec<String> = Vec::new();
    let mut success_resp: Vec<Value> = Vec::new();

    let messages: WebhookMessagePayload = serde_json::from_str(message).unwrap();

    for (entry_index, entry) in messages.entry.iter().enumerate() {
        if entry.id.trim().trim_matches('"') != config.whatsapp_business_id {
            error_resp.push(format!("Entry id not recognized index: {}", entry_index));
            continue;
        }
        for (change_index, change) in entry.changes.iter().enumerate() {
            if change
                .value
                .metadata
                .phone_number_id
                .trim()
                .trim_matches('"')
                != config.whatsapp_phone_number_id
            {
                error_resp.push(format!(
                    "Metadata phone_number_id not recognized index: {}",
                    change_index
                ));
                continue;
            }
            for message in change.value.messages.iter() {
                if let Some(r#type) = message.get("type") {
                    if r#type == "order" {
                        let order = Order {
                            from: message
                                .get("from")
                                .and_then(|v| v.as_str())
                                .unwrap_or_default()
                                .to_string(),

                            id: message
                                .get("id")
                                .and_then(|v| v.as_str())
                                .unwrap_or_default()
                                .to_string(),

                            context: {
                                let context =
                                    message.get("context").unwrap_or(&serde_json::Value::Null);
                                MessageContext {
                                    from: context
                                        .get("from")
                                        .and_then(|v| v.as_str())
                                        .unwrap_or_default()
                                        .to_string(),

                                    id: context
                                        .get("id")
                                        .and_then(|v| v.as_str())
                                        .unwrap_or_default()
                                        .to_string(),
                                }
                            },

                            r#type: message
                                .get("type")
                                .and_then(|v| v.as_str())
                                .unwrap_or_default()
                                .to_string(),

                            timestamp: message
                                .get("timestamp")
                                .and_then(|v| v.as_str())
                                .unwrap_or_default()
                                .to_string(),

                            order: {
                                let order_details =
                                    message.get("order").unwrap_or(&serde_json::Value::Null);
                                OrderDetails {
                                    catalog_id: order_details
                                        .get("catalog_id")
                                        .and_then(|v| v.as_str())
                                        .unwrap_or_default()
                                        .to_string(),

                                    product_items: {
                                        let product_items = order_details
                                            .get("product_items")
                                            .unwrap_or(&serde_json::Value::Null);
                                        product_items
                                            .as_array()
                                            .unwrap_or(&vec![])
                                            .iter()
                                            .map(|item| ProductItem {
                                                product_retailer_id: item
                                                    .get("product_retailer_id")
                                                    .and_then(|v| v.as_str())
                                                    .unwrap_or_default()
                                                    .to_string(),

                                                quantity: item
                                                    .get("quantity")
                                                    .and_then(|v| v.as_str())
                                                    .unwrap_or_default()
                                                    .to_string(),

                                                item_price: item
                                                    .get("item_price")
                                                    .and_then(|v| v.as_str())
                                                    .unwrap_or_default()
                                                    .to_string(),

                                                currency: item
                                                    .get("currency")
                                                    .and_then(|v| v.as_str())
                                                    .unwrap_or_default()
                                                    .to_string(),
                                            })
                                            .collect::<Vec<_>>()
                                    },

                                    text: order_details
                                        .get("text")
                                        .and_then(|v| v.as_str())
                                        .unwrap_or_default()
                                        .to_string(),
                                }
                            },
                        };

                        let payload = json!(order);

                        success_resp.push(payload)
                    } else if r#type == "text" {
                        let text_message = Text {
                            from: message
                                .get("from")
                                .and_then(|v| v.as_str())
                                .unwrap_or_default()
                                .to_string(),

                            id: message
                                .get("id")
                                .and_then(|v| v.as_str())
                                .unwrap_or_default()
                                .to_string(),

                            timestamp: message
                                .get("timestamp")
                                .and_then(|v| v.as_str())
                                .unwrap_or_default()
                                .to_string(),

                            r#type: message
                                .get("type")
                                .and_then(|v| v.as_str())
                                .unwrap_or_default()
                                .to_string(),

                            text: TextPayload {
                                body: message
                                    .get("text")
                                    .and_then(|text_obj| text_obj.get("body"))
                                    .and_then(|v| v.as_str())
                                    .unwrap_or_default()
                                    .to_string(),
                            },
                        };

                        let payload = json!(text_message);

                        success_resp.push(payload);
                    } else if r#type == "unknown" {
                        let unknown_message = Unknown {
                            from: message
                                .get("from")
                                .and_then(|v| v.as_str())
                                .unwrap_or_default()
                                .to_string(),

                            id: message
                                .get("id")
                                .and_then(|v| v.as_str())
                                .unwrap_or_default()
                                .to_string(),

                            timestamp: message
                                .get("timestamp")
                                .and_then(|v| v.as_str())
                                .unwrap_or_default()
                                .to_string(),

                            r#type: message
                                .get("type")
                                .and_then(|v| v.as_str())
                                .unwrap_or_default()
                                .to_string(),

                            errors: message
                                .get("errors")
                                .and_then(|v| v.as_array())
                                .map(|arr| {
                                    arr.iter()
                                        .map(|err| {
                                            ErrorDetail {
                                                code: err
                                                    .get("code")
                                                    .and_then(|v| v.as_u64())
                                                    .unwrap_or(0)
                                                    as u32, // assuming `code` is u32

                                                details: err
                                                    .get("details")
                                                    .and_then(|v| v.as_str())
                                                    .unwrap_or_default()
                                                    .to_string(),

                                                title: err
                                                    .get("title")
                                                    .and_then(|v| v.as_str())
                                                    .unwrap_or_default()
                                                    .to_string(),
                                            }
                                        })
                                        .collect()
                                })
                                .unwrap_or_else(Vec::new),
                        };

                        let payload = json!(unknown_message);

                        success_resp.push(payload);
                    } else if r#type == "location" {
                        let location_message = Location {
                            from: message
                                .get("from")
                                .and_then(|v| v.as_str())
                                .unwrap_or_default()
                                .to_string(),

                            id: message
                                .get("id")
                                .and_then(|v| v.as_str())
                                .unwrap_or_default()
                                .to_string(),

                            timestamp: message
                                .get("timestamp")
                                .and_then(|v| v.as_str())
                                .unwrap_or_default()
                                .to_string(),

                            location: {
                                let loc =
                                    message.get("location").unwrap_or(&serde_json::Value::Null);
                                LocationPayload {
                                    latitude: loc
                                        .get("latitude")
                                        .and_then(|v| v.as_str())
                                        .unwrap_or_default()
                                        .to_string(),

                                    longitude: loc
                                        .get("longitude")
                                        .and_then(|v| v.as_str())
                                        .unwrap_or_default()
                                        .to_string(),

                                    name: loc
                                        .get("name")
                                        .and_then(|v| v.as_str())
                                        .unwrap_or_default()
                                        .to_string(),

                                    address: loc
                                        .get("address")
                                        .and_then(|v| v.as_str())
                                        .unwrap_or_default()
                                        .to_string(),
                                }
                            },
                        };

                        let payload = json!(location_message);

                        success_resp.push(payload);
                    } else if r#type == "contacts" {
                        let contacts_message = Contact {
                            addresses: {
                                let addresses =
                                    message.get("addresses").unwrap_or(&serde_json::Value::Null);
                                addresses
                                    .as_array()
                                    .unwrap_or(&vec![])
                                    .iter()
                                    .map(|addr| Address {
                                        city: addr
                                            .get("city")
                                            .and_then(|v| v.as_str())
                                            .unwrap_or_default()
                                            .to_string(),

                                        country: addr
                                            .get("country")
                                            .and_then(|v| v.as_str())
                                            .unwrap_or_default()
                                            .to_string(),

                                        country_code: addr
                                            .get("country_code")
                                            .and_then(|v| v.as_str())
                                            .unwrap_or_default()
                                            .to_string(),

                                        state: addr
                                            .get("state")
                                            .and_then(|v| v.as_str())
                                            .unwrap_or_default()
                                            .to_string(),

                                        street: addr
                                            .get("street")
                                            .and_then(|v| v.as_str())
                                            .unwrap_or_default()
                                            .to_string(),

                                        r#type: addr
                                            .get("type")
                                            .and_then(|v| v.as_str())
                                            .unwrap_or_default()
                                            .to_string(),

                                        zip: addr
                                            .get("zip")
                                            .and_then(|v| v.as_str())
                                            .unwrap_or_default()
                                            .to_string(),
                                    })
                                    .collect::<Vec<_>>()
                            },

                            birthday: message
                                .get("birthday")
                                .and_then(|v| v.as_str())
                                .unwrap_or_default()
                                .to_string(),

                            emails: {
                                let emails =
                                    message.get("emails").unwrap_or(&serde_json::Value::Null);
                                emails
                                    .as_array()
                                    .unwrap_or(&vec![])
                                    .iter()
                                    .map(|email| Email {
                                        email: email
                                            .get("email")
                                            .and_then(|v| v.as_str())
                                            .unwrap_or_default()
                                            .to_string(),

                                        r#type: email
                                            .get("type")
                                            .and_then(|v| v.as_str())
                                            .unwrap_or_default()
                                            .to_string(),
                                    })
                                    .collect::<Vec<_>>()
                            },

                            name: {
                                let name = message.get("name").unwrap_or(&serde_json::Value::Null);
                                Name {
                                    formatted_name: name
                                        .get("formatted_name")
                                        .and_then(|v| v.as_str())
                                        .unwrap_or_default()
                                        .to_string(),

                                    first_name: name
                                        .get("first_name")
                                        .and_then(|v| v.as_str())
                                        .unwrap_or_default()
                                        .to_string(),

                                    last_name: name
                                        .get("last_name")
                                        .and_then(|v| v.as_str())
                                        .unwrap_or_default()
                                        .to_string(),

                                    middle_name: name
                                        .get("middle_name")
                                        .and_then(|v| v.as_str())
                                        .unwrap_or_default()
                                        .to_string(),

                                    suffix: name
                                        .get("suffix")
                                        .and_then(|v| v.as_str())
                                        .unwrap_or_default()
                                        .to_string(),

                                    prefix: name
                                        .get("prefix")
                                        .and_then(|v| v.as_str())
                                        .unwrap_or_default()
                                        .to_string(),
                                }
                            },

                            org: {
                                let org = message.get("org").unwrap_or(&serde_json::Value::Null);
                                Organization {
                                    company: org
                                        .get("company")
                                        .and_then(|v| v.as_str())
                                        .unwrap_or_default()
                                        .to_string(),

                                    department: org
                                        .get("department")
                                        .and_then(|v| v.as_str())
                                        .unwrap_or_default()
                                        .to_string(),

                                    title: org
                                        .get("title")
                                        .and_then(|v| v.as_str())
                                        .unwrap_or_default()
                                        .to_string(),
                                }
                            },

                            phones: {
                                let phones =
                                    message.get("phones").unwrap_or(&serde_json::Value::Null);
                                phones
                                    .as_array()
                                    .unwrap_or(&vec![])
                                    .iter()
                                    .map(|phone| Phone {
                                        phone: phone
                                            .get("phone")
                                            .and_then(|v| v.as_str())
                                            .unwrap_or_default()
                                            .to_string(),

                                        wa_id: phone
                                            .get("wa_id")
                                            .and_then(|v| v.as_str())
                                            .unwrap_or_default()
                                            .to_string(),

                                        r#type: phone
                                            .get("type")
                                            .and_then(|v| v.as_str())
                                            .unwrap_or_default()
                                            .to_string(),
                                    })
                                    .collect::<Vec<_>>()
                            },

                            urls: {
                                let urls = message.get("urls").unwrap_or(&serde_json::Value::Null);
                                urls.as_array()
                                    .unwrap_or(&vec![])
                                    .iter()
                                    .map(|url| Url {
                                        url: url
                                            .get("url")
                                            .and_then(|v| v.as_str())
                                            .unwrap_or_default()
                                            .to_string(),

                                        r#type: url
                                            .get("type")
                                            .and_then(|v| v.as_str())
                                            .unwrap_or_default()
                                            .to_string(),
                                    })
                                    .collect::<Vec<_>>()
                            },
                        };

                        let payload = json!(contacts_message);

                        success_resp.push(payload);
                    } else if r#type == "reaction" {
                        let reaction_message = Reaction {
                            from: message
                                .get("from")
                                .and_then(|v| v.as_str())
                                .unwrap_or_default()
                                .to_string(),

                            id: message
                                .get("id")
                                .and_then(|v| v.as_str())
                                .unwrap_or_default()
                                .to_string(),

                            timestamp: message
                                .get("timestamp")
                                .and_then(|v| v.as_str())
                                .unwrap_or_default()
                                .to_string(),

                            r#type: message
                                .get("type")
                                .and_then(|v| v.as_str())
                                .unwrap_or_else(|| "reaction")
                                .to_string(),

                            reaction: {
                                let react =
                                    message.get("reaction").unwrap_or(&serde_json::Value::Null);
                                ReactionPayload {
                                    emoji: react
                                        .get("emoji")
                                        .and_then(|v| v.as_str())
                                        .unwrap_or_default()
                                        .to_string(),

                                    message_id: react
                                        .get("message_id")
                                        .and_then(|v| v.as_str())
                                        .unwrap_or_default()
                                        .to_string(),
                                }
                            },
                        };

                        let payload = json!(reaction_message);

                        success_resp.push(payload);
                    } else if r#type == "button" {
                        let button_message = Button {
                            from: message
                                .get("from")
                                .and_then(|v| v.as_str())
                                .unwrap_or_default()
                                .to_string(),

                            id: message
                                .get("id")
                                .and_then(|v| v.as_str())
                                .unwrap_or_default()
                                .to_string(),

                            timestamp: message
                                .get("timestamp")
                                .and_then(|v| v.as_str())
                                .unwrap_or_default()
                                .to_string(),

                            r#type: message
                                .get("type")
                                .and_then(|v| v.as_str())
                                .unwrap_or_default()
                                .to_string(),

                            context: {
                                let context =
                                    message.get("context").unwrap_or(&serde_json::Value::Null);
                                MsgBtnContext {
                                    from: context
                                        .get("from")
                                        .and_then(|v| v.as_str())
                                        .unwrap_or_default()
                                        .to_string(),

                                    id: context
                                        .get("id")
                                        .and_then(|v| v.as_str())
                                        .unwrap_or_default()
                                        .to_string(),
                                }
                            },

                            button: {
                                let button_details =
                                    message.get("button").unwrap_or(&serde_json::Value::Null);
                                ButtonDetails {
                                    text: button_details
                                        .get("text")
                                        .and_then(|v| v.as_str())
                                        .unwrap_or_default()
                                        .to_string(),

                                    payload: button_details
                                        .get("payload")
                                        .and_then(|v| v.as_str())
                                        .unwrap_or_default()
                                        .to_string(),
                                }
                            },
                        };

                        let payload = json!(button_message);

                        success_resp.push(payload);
                    } else if r#type == "sticker" {
                        let sticker_message = Sticker {
                            from: message
                                .get("from")
                                .and_then(|v| v.as_str())
                                .unwrap_or_default()
                                .to_string(),

                            id: message
                                .get("id")
                                .and_then(|v| v.as_str())
                                .unwrap_or_default()
                                .to_string(),

                            timestamp: message
                                .get("timestamp")
                                .and_then(|v| v.as_str())
                                .unwrap_or_default()
                                .to_string(),

                            r#type: message
                                .get("type")
                                .and_then(|v| v.as_str())
                                .unwrap_or_else(|| "sticker")
                                .to_string(),

                            sticker: {
                                let sticker =
                                    message.get("sticker").unwrap_or(&serde_json::Value::Null);
                                StickerData {
                                    id: sticker
                                        .get("id")
                                        .and_then(|v| v.as_str())
                                        .unwrap_or_default()
                                        .to_string(),

                                    animated: sticker
                                        .get("animated")
                                        .and_then(|v| v.as_bool())
                                        .unwrap_or(false),

                                    mime_type: sticker
                                        .get("mime_type")
                                        .and_then(|v| v.as_str())
                                        .unwrap_or_default()
                                        .to_string(),

                                    sha256: sticker
                                        .get("sha256")
                                        .and_then(|v| v.as_str())
                                        .unwrap_or_default()
                                        .to_string(),
                                }
                            },
                        };

                        let payload = json!(sticker_message);

                        success_resp.push(payload);
                    } else if r#type == "video" {
                        let video_message = Video {
                            from: message
                                .get("from")
                                .and_then(|v| v.as_str())
                                .unwrap_or_default()
                                .to_string(),

                            id: message
                                .get("id")
                                .and_then(|v| v.as_str())
                                .unwrap_or_default()
                                .to_string(),

                            timestamp: message
                                .get("timestamp")
                                .and_then(|v| v.as_str())
                                .unwrap_or_default()
                                .to_string(),

                            r#type: message
                                .get("type")
                                .and_then(|v| v.as_str())
                                .unwrap_or_else(|| "video")
                                .to_string(),

                            document: {
                                let document =
                                    message.get("document").unwrap_or(&serde_json::Value::Null);
                                VideoData {
                                    caption: document
                                        .get("caption")
                                        .and_then(|v| v.as_str())
                                        .unwrap_or_default()
                                        .to_string(),

                                    mime_type: document
                                        .get("mime_type")
                                        .and_then(|v| v.as_str())
                                        .unwrap_or_default()
                                        .to_string(),

                                    sha256: document
                                        .get("sha256")
                                        .and_then(|v| v.as_str())
                                        .unwrap_or_default()
                                        .to_string(),

                                    id: document
                                        .get("id")
                                        .and_then(|v| v.as_str())
                                        .unwrap_or_default()
                                        .to_string(),
                                }
                            },
                        };

                        let payload = json!(video_message);

                        success_resp.push(payload);
                    } else if r#type == "audio" {
                        let audio_message = Audio {
                            from: message
                                .get("from")
                                .and_then(|v| v.as_str())
                                .unwrap_or_default()
                                .to_string(),

                            id: message
                                .get("id")
                                .and_then(|v| v.as_str())
                                .unwrap_or_default()
                                .to_string(),

                            timestamp: message
                                .get("timestamp")
                                .and_then(|v| v.as_str())
                                .unwrap_or_default()
                                .to_string(),

                            r#type: message
                                .get("type")
                                .and_then(|v| v.as_str())
                                .unwrap_or_else(|| "audio")
                                .to_string(),

                            document: {
                                let document =
                                    message.get("document").unwrap_or(&serde_json::Value::Null);
                                AudioData {
                                    caption: document
                                        .get("caption")
                                        .and_then(|v| v.as_str())
                                        .unwrap_or_default()
                                        .to_string(),

                                    mime_type: document
                                        .get("mime_type")
                                        .and_then(|v| v.as_str())
                                        .unwrap_or_default()
                                        .to_string(),

                                    sha256: document
                                        .get("sha256")
                                        .and_then(|v| v.as_str())
                                        .unwrap_or_default()
                                        .to_string(),

                                    id: document
                                        .get("id")
                                        .and_then(|v| v.as_str())
                                        .unwrap_or_default()
                                        .to_string(),
                                }
                            },
                        };

                        let payload = json!(audio_message);

                        success_resp.push(payload);
                    } else if r#type == "document" {
                        let document_message = Document {
                            from: message
                                .get("from")
                                .and_then(|v| v.as_str())
                                .unwrap_or_default()
                                .to_string(),

                            id: message
                                .get("id")
                                .and_then(|v| v.as_str())
                                .unwrap_or_default()
                                .to_string(),

                            timestamp: message
                                .get("timestamp")
                                .and_then(|v| v.as_str())
                                .unwrap_or_default()
                                .to_string(),

                            r#type: message
                                .get("type")
                                .and_then(|v| v.as_str())
                                .unwrap_or_else(|| "document")
                                .to_string(),

                            document: {
                                let document =
                                    message.get("document").unwrap_or(&serde_json::Value::Null);
                                DocumentData {
                                    caption: document
                                        .get("caption")
                                        .and_then(|v| v.as_str())
                                        .unwrap_or_default()
                                        .to_string(),

                                    mime_type: document
                                        .get("mime_type")
                                        .and_then(|v| v.as_str())
                                        .unwrap_or_default()
                                        .to_string(),

                                    sha256: document
                                        .get("sha256")
                                        .and_then(|v| v.as_str())
                                        .unwrap_or_default()
                                        .to_string(),

                                    id: document
                                        .get("id")
                                        .and_then(|v| v.as_str())
                                        .unwrap_or_default()
                                        .to_string(),
                                }
                            },
                        };

                        let payload = json!(document_message);

                        success_resp.push(payload);
                    } else if r#type == "image" {
                        let image_message = Image {
                            from: message
                                .get("from")
                                .and_then(|v| v.as_str())
                                .unwrap_or_default()
                                .to_string(),

                            id: message
                                .get("id")
                                .and_then(|v| v.as_str())
                                .unwrap_or_default()
                                .to_string(),

                            timestamp: message
                                .get("timestamp")
                                .and_then(|v| v.as_str())
                                .unwrap_or_default()
                                .to_string(),

                            r#type: message
                                .get("type")
                                .and_then(|v| v.as_str())
                                .unwrap_or_else(|| "image")
                                .to_string(),

                            image: {
                                let image =
                                    message.get("image").unwrap_or(&serde_json::Value::Null);
                                ImageData {
                                    caption: image
                                        .get("caption")
                                        .and_then(|v| v.as_str())
                                        .unwrap_or_default()
                                        .to_string(),

                                    mime_type: image
                                        .get("mime_type")
                                        .and_then(|v| v.as_str())
                                        .unwrap_or_default()
                                        .to_string(),

                                    sha256: image
                                        .get("sha256")
                                        .and_then(|v| v.as_str())
                                        .unwrap_or_default()
                                        .to_string(),

                                    id: image
                                        .get("id")
                                        .and_then(|v| v.as_str())
                                        .unwrap_or_default()
                                        .to_string(),
                                }
                            },
                        };

                        let payload = json!(image_message);

                        success_resp.push(payload);
                    } else {
                        error_resp.push("Message format could not be recognized".to_string());
                        continue;
                    }
                }
            }
        }
    }

    Ok((success_resp, error_resp))

    ////////////////////////////////////

    // let message: Value = serde_json::from_str(message).map_err(|e| e.to_string())?;

    // let entries = message
    //     .get("entry")
    //     .map(|value| value.as_array())
    //     .unwrap_or_default();

    // if entries.is_none() {
    //     return Err("Entry data not found".to_string());
    // }

    // for (index, entry) in entries.unwrap().iter().enumerate() {
    //     let entry_id = entry
    //         .get("id")
    //         .map(|value| value.to_string())
    //         .unwrap_or_default();

    //     if entry_id.trim().trim_matches('"') != config.whatsapp_business_id.trim() {
    //         return Err(format!(
    //             "Whatsapp_business_id does not tally with config whatsapp_business_id: {}",
    //             index
    //         )
    //         .to_string());
    //     }

    //     if let Some(changes) = entry.get("changes").and_then(|c| c.as_array()) {
    //         for change in changes {
    //             let value = change.get("value").and_then(|v| v.as_object());

    //             if value.is_none() {
    //                 return Err(format!("Value not found: {} index: ", index).to_string());
    //             }

    //             let phone_number_id = value
    //                 .and_then(|v| v.get("metadata"))
    //                 .and_then(|m| m.get("phone_number_id"))
    //                 .and_then(|id| id.as_str())
    //                 .unwrap_or_default();

    //             if phone_number_id.trim().trim_matches('"')
    //                 != config.whatsapp_phone_number_id.trim()
    //             {
    //                 return Err(format!("Message whatsapp_phone_number_id does not tally with config whatsapp_phone_number_id").to_string());
    //             }

    //             if let Some(messages) = value
    //                 .and_then(|v| v.get("messages"))
    //                 .and_then(|m| m.as_array())
    //             {
    //                 for message in messages {
    //                     // identify message type

    //                     if let Some(r#type) = message.get("type") {
    //                         if r#type == "order" {
    //                             let order = Order {
    //                                 from: message
    //                                     .get("from")
    //                                     .and_then(|v| v.as_str())
    //                                     .unwrap_or_default()
    //                                     .to_string(),

    //                                 id: message
    //                                     .get("id")
    //                                     .and_then(|v| v.as_str())
    //                                     .unwrap_or_default()
    //                                     .to_string(),

    //                                 context: {
    //                                     let context = message
    //                                         .get("context")
    //                                         .unwrap_or(&serde_json::Value::Null);
    //                                     MessageContext {
    //                                         from: context
    //                                             .get("from")
    //                                             .and_then(|v| v.as_str())
    //                                             .unwrap_or_default()
    //                                             .to_string(),

    //                                         id: context
    //                                             .get("id")
    //                                             .and_then(|v| v.as_str())
    //                                             .unwrap_or_default()
    //                                             .to_string(),
    //                                     }
    //                                 },

    //                                 r#type: message
    //                                     .get("type")
    //                                     .and_then(|v| v.as_str())
    //                                     .unwrap_or_default()
    //                                     .to_string(),

    //                                 timestamp: message
    //                                     .get("timestamp")
    //                                     .and_then(|v| v.as_str())
    //                                     .unwrap_or_default()
    //                                     .to_string(),

    //                                 order: {
    //                                     let order_details = message
    //                                         .get("order")
    //                                         .unwrap_or(&serde_json::Value::Null);
    //                                     OrderDetails {
    //                                         catalog_id: order_details
    //                                             .get("catalog_id")
    //                                             .and_then(|v| v.as_str())
    //                                             .unwrap_or_default()
    //                                             .to_string(),

    //                                         product_items: {
    //                                             let product_items = order_details
    //                                                 .get("product_items")
    //                                                 .unwrap_or(&serde_json::Value::Null);
    //                                             product_items
    //                                                 .as_array()
    //                                                 .unwrap_or(&vec![])
    //                                                 .iter()
    //                                                 .map(|item| ProductItem {
    //                                                     product_retailer_id: item
    //                                                         .get("product_retailer_id")
    //                                                         .and_then(|v| v.as_str())
    //                                                         .unwrap_or_default()
    //                                                         .to_string(),

    //                                                     quantity: item
    //                                                         .get("quantity")
    //                                                         .and_then(|v| v.as_str())
    //                                                         .unwrap_or_default()
    //                                                         .to_string(),

    //                                                     item_price: item
    //                                                         .get("item_price")
    //                                                         .and_then(|v| v.as_str())
    //                                                         .unwrap_or_default()
    //                                                         .to_string(),

    //                                                     currency: item
    //                                                         .get("currency")
    //                                                         .and_then(|v| v.as_str())
    //                                                         .unwrap_or_default()
    //                                                         .to_string(),
    //                                                 })
    //                                                 .collect::<Vec<_>>()
    //                                         },

    //                                         text: order_details
    //                                             .get("text")
    //                                             .and_then(|v| v.as_str())
    //                                             .unwrap_or_default()
    //                                             .to_string(),
    //                                     }
    //                                 },
    //                             };

    //                             let payload = json!(order);

    //                             message_payload.push(payload)
    //                         } else if r#type == "text" {
    //                             let text_message = Text {
    //                                 from: message
    //                                     .get("from")
    //                                     .and_then(|v| v.as_str())
    //                                     .unwrap_or_default()
    //                                     .to_string(),

    //                                 id: message
    //                                     .get("id")
    //                                     .and_then(|v| v.as_str())
    //                                     .unwrap_or_default()
    //                                     .to_string(),

    //                                 timestamp: message
    //                                     .get("timestamp")
    //                                     .and_then(|v| v.as_str())
    //                                     .unwrap_or_default()
    //                                     .to_string(),

    //                                 r#type: message
    //                                     .get("type")
    //                                     .and_then(|v| v.as_str())
    //                                     .unwrap_or_default()
    //                                     .to_string(),

    //                                 text: TextPayload {
    //                                     body: message
    //                                         .get("text")
    //                                         .and_then(|text_obj| text_obj.get("body"))
    //                                         .and_then(|v| v.as_str())
    //                                         .unwrap_or_default()
    //                                         .to_string(),
    //                                 },
    //                             };

    //                             let payload = json!(text_message);

    //                             message_payload.push(payload);
    //                         } else if r#type == "unknown" {
    //                             let unknown_message = Unknown {
    //                                 from: message
    //                                     .get("from")
    //                                     .and_then(|v| v.as_str())
    //                                     .unwrap_or_default()
    //                                     .to_string(),

    //                                 id: message
    //                                     .get("id")
    //                                     .and_then(|v| v.as_str())
    //                                     .unwrap_or_default()
    //                                     .to_string(),

    //                                 timestamp: message
    //                                     .get("timestamp")
    //                                     .and_then(|v| v.as_str())
    //                                     .unwrap_or_default()
    //                                     .to_string(),

    //                                 r#type: message
    //                                     .get("type")
    //                                     .and_then(|v| v.as_str())
    //                                     .unwrap_or_default()
    //                                     .to_string(),

    //                                 errors: message
    //                                     .get("errors")
    //                                     .and_then(|v| v.as_array())
    //                                     .map(|arr| {
    //                                         arr.iter()
    //                                             .map(|err| {
    //                                                 ErrorDetail {
    //                                                     code: err
    //                                                         .get("code")
    //                                                         .and_then(|v| v.as_u64())
    //                                                         .unwrap_or(0)
    //                                                         as u32, // assuming `code` is u32

    //                                                     details: err
    //                                                         .get("details")
    //                                                         .and_then(|v| v.as_str())
    //                                                         .unwrap_or_default()
    //                                                         .to_string(),

    //                                                     title: err
    //                                                         .get("title")
    //                                                         .and_then(|v| v.as_str())
    //                                                         .unwrap_or_default()
    //                                                         .to_string(),
    //                                                 }
    //                                             })
    //                                             .collect()
    //                                     })
    //                                     .unwrap_or_else(Vec::new),
    //                             };

    //                             let payload = json!(unknown_message);

    //                             message_payload.push(payload);
    //                         } else if r#type == "location" {
    //                             let location_message = Location {
    //                                 from: message
    //                                     .get("from")
    //                                     .and_then(|v| v.as_str())
    //                                     .unwrap_or_default()
    //                                     .to_string(),

    //                                 id: message
    //                                     .get("id")
    //                                     .and_then(|v| v.as_str())
    //                                     .unwrap_or_default()
    //                                     .to_string(),

    //                                 timestamp: message
    //                                     .get("timestamp")
    //                                     .and_then(|v| v.as_str())
    //                                     .unwrap_or_default()
    //                                     .to_string(),

    //                                 location: {
    //                                     let loc = message
    //                                         .get("location")
    //                                         .unwrap_or(&serde_json::Value::Null);
    //                                     LocationPayload {
    //                                         latitude: loc
    //                                             .get("latitude")
    //                                             .and_then(|v| v.as_str())
    //                                             .unwrap_or_default()
    //                                             .to_string(),

    //                                         longitude: loc
    //                                             .get("longitude")
    //                                             .and_then(|v| v.as_str())
    //                                             .unwrap_or_default()
    //                                             .to_string(),

    //                                         name: loc
    //                                             .get("name")
    //                                             .and_then(|v| v.as_str())
    //                                             .unwrap_or_default()
    //                                             .to_string(),

    //                                         address: loc
    //                                             .get("address")
    //                                             .and_then(|v| v.as_str())
    //                                             .unwrap_or_default()
    //                                             .to_string(),
    //                                     }
    //                                 },
    //                             };

    //                             let payload = json!(location_message);

    //                             message_payload.push(payload);
    //                         } else if r#type == "contacts" {
    //                             let contacts_message = Contact {
    //                                 addresses: {
    //                                     let addresses = message
    //                                         .get("addresses")
    //                                         .unwrap_or(&serde_json::Value::Null);
    //                                     addresses
    //                                         .as_array()
    //                                         .unwrap_or(&vec![])
    //                                         .iter()
    //                                         .map(|addr| Address {
    //                                             city: addr
    //                                                 .get("city")
    //                                                 .and_then(|v| v.as_str())
    //                                                 .unwrap_or_default()
    //                                                 .to_string(),

    //                                             country: addr
    //                                                 .get("country")
    //                                                 .and_then(|v| v.as_str())
    //                                                 .unwrap_or_default()
    //                                                 .to_string(),

    //                                             country_code: addr
    //                                                 .get("country_code")
    //                                                 .and_then(|v| v.as_str())
    //                                                 .unwrap_or_default()
    //                                                 .to_string(),

    //                                             state: addr
    //                                                 .get("state")
    //                                                 .and_then(|v| v.as_str())
    //                                                 .unwrap_or_default()
    //                                                 .to_string(),

    //                                             street: addr
    //                                                 .get("street")
    //                                                 .and_then(|v| v.as_str())
    //                                                 .unwrap_or_default()
    //                                                 .to_string(),

    //                                             r#type: addr
    //                                                 .get("type")
    //                                                 .and_then(|v| v.as_str())
    //                                                 .unwrap_or_default()
    //                                                 .to_string(),

    //                                             zip: addr
    //                                                 .get("zip")
    //                                                 .and_then(|v| v.as_str())
    //                                                 .unwrap_or_default()
    //                                                 .to_string(),
    //                                         })
    //                                         .collect::<Vec<_>>()
    //                                 },

    //                                 birthday: message
    //                                     .get("birthday")
    //                                     .and_then(|v| v.as_str())
    //                                     .unwrap_or_default()
    //                                     .to_string(),

    //                                 emails: {
    //                                     let emails = message
    //                                         .get("emails")
    //                                         .unwrap_or(&serde_json::Value::Null);
    //                                     emails
    //                                         .as_array()
    //                                         .unwrap_or(&vec![])
    //                                         .iter()
    //                                         .map(|email| Email {
    //                                             email: email
    //                                                 .get("email")
    //                                                 .and_then(|v| v.as_str())
    //                                                 .unwrap_or_default()
    //                                                 .to_string(),

    //                                             r#type: email
    //                                                 .get("type")
    //                                                 .and_then(|v| v.as_str())
    //                                                 .unwrap_or_default()
    //                                                 .to_string(),
    //                                         })
    //                                         .collect::<Vec<_>>()
    //                                 },

    //                                 name: {
    //                                     let name =
    //                                         message.get("name").unwrap_or(&serde_json::Value::Null);
    //                                     Name {
    //                                         formatted_name: name
    //                                             .get("formatted_name")
    //                                             .and_then(|v| v.as_str())
    //                                             .unwrap_or_default()
    //                                             .to_string(),

    //                                         first_name: name
    //                                             .get("first_name")
    //                                             .and_then(|v| v.as_str())
    //                                             .unwrap_or_default()
    //                                             .to_string(),

    //                                         last_name: name
    //                                             .get("last_name")
    //                                             .and_then(|v| v.as_str())
    //                                             .unwrap_or_default()
    //                                             .to_string(),

    //                                         middle_name: name
    //                                             .get("middle_name")
    //                                             .and_then(|v| v.as_str())
    //                                             .unwrap_or_default()
    //                                             .to_string(),

    //                                         suffix: name
    //                                             .get("suffix")
    //                                             .and_then(|v| v.as_str())
    //                                             .unwrap_or_default()
    //                                             .to_string(),

    //                                         prefix: name
    //                                             .get("prefix")
    //                                             .and_then(|v| v.as_str())
    //                                             .unwrap_or_default()
    //                                             .to_string(),
    //                                     }
    //                                 },

    //                                 org: {
    //                                     let org =
    //                                         message.get("org").unwrap_or(&serde_json::Value::Null);
    //                                     Organization {
    //                                         company: org
    //                                             .get("company")
    //                                             .and_then(|v| v.as_str())
    //                                             .unwrap_or_default()
    //                                             .to_string(),

    //                                         department: org
    //                                             .get("department")
    //                                             .and_then(|v| v.as_str())
    //                                             .unwrap_or_default()
    //                                             .to_string(),

    //                                         title: org
    //                                             .get("title")
    //                                             .and_then(|v| v.as_str())
    //                                             .unwrap_or_default()
    //                                             .to_string(),
    //                                     }
    //                                 },

    //                                 phones: {
    //                                     let phones = message
    //                                         .get("phones")
    //                                         .unwrap_or(&serde_json::Value::Null);
    //                                     phones
    //                                         .as_array()
    //                                         .unwrap_or(&vec![])
    //                                         .iter()
    //                                         .map(|phone| Phone {
    //                                             phone: phone
    //                                                 .get("phone")
    //                                                 .and_then(|v| v.as_str())
    //                                                 .unwrap_or_default()
    //                                                 .to_string(),

    //                                             wa_id: phone
    //                                                 .get("wa_id")
    //                                                 .and_then(|v| v.as_str())
    //                                                 .unwrap_or_default()
    //                                                 .to_string(),

    //                                             r#type: phone
    //                                                 .get("type")
    //                                                 .and_then(|v| v.as_str())
    //                                                 .unwrap_or_default()
    //                                                 .to_string(),
    //                                         })
    //                                         .collect::<Vec<_>>()
    //                                 },

    //                                 urls: {
    //                                     let urls =
    //                                         message.get("urls").unwrap_or(&serde_json::Value::Null);
    //                                     urls.as_array()
    //                                         .unwrap_or(&vec![])
    //                                         .iter()
    //                                         .map(|url| Url {
    //                                             url: url
    //                                                 .get("url")
    //                                                 .and_then(|v| v.as_str())
    //                                                 .unwrap_or_default()
    //                                                 .to_string(),

    //                                             r#type: url
    //                                                 .get("type")
    //                                                 .and_then(|v| v.as_str())
    //                                                 .unwrap_or_default()
    //                                                 .to_string(),
    //                                         })
    //                                         .collect::<Vec<_>>()
    //                                 },
    //                             };

    //                             let payload = json!(contacts_message);

    //                             message_payload.push(payload);
    //                         } else if r#type == "reaction" {
    //                             let reaction_message = Reaction {
    //                                 from: message
    //                                     .get("from")
    //                                     .and_then(|v| v.as_str())
    //                                     .unwrap_or_default()
    //                                     .to_string(),

    //                                 id: message
    //                                     .get("id")
    //                                     .and_then(|v| v.as_str())
    //                                     .unwrap_or_default()
    //                                     .to_string(),

    //                                 timestamp: message
    //                                     .get("timestamp")
    //                                     .and_then(|v| v.as_str())
    //                                     .unwrap_or_default()
    //                                     .to_string(),

    //                                 r#type: message
    //                                     .get("type")
    //                                     .and_then(|v| v.as_str())
    //                                     .unwrap_or_else(|| "reaction")
    //                                     .to_string(),

    //                                 reaction: {
    //                                     let react = message
    //                                         .get("reaction")
    //                                         .unwrap_or(&serde_json::Value::Null);
    //                                     ReactionPayload {
    //                                         emoji: react
    //                                             .get("emoji")
    //                                             .and_then(|v| v.as_str())
    //                                             .unwrap_or_default()
    //                                             .to_string(),

    //                                         message_id: react
    //                                             .get("message_id")
    //                                             .and_then(|v| v.as_str())
    //                                             .unwrap_or_default()
    //                                             .to_string(),
    //                                     }
    //                                 },
    //                             };

    //                             let payload = json!(reaction_message);

    //                             message_payload.push(payload);
    //                         } else if r#type == "button" {
    //                             let button_message = Button {
    //                                 from: message
    //                                     .get("from")
    //                                     .and_then(|v| v.as_str())
    //                                     .unwrap_or_default()
    //                                     .to_string(),

    //                                 id: message
    //                                     .get("id")
    //                                     .and_then(|v| v.as_str())
    //                                     .unwrap_or_default()
    //                                     .to_string(),

    //                                 timestamp: message
    //                                     .get("timestamp")
    //                                     .and_then(|v| v.as_str())
    //                                     .unwrap_or_default()
    //                                     .to_string(),

    //                                 r#type: message
    //                                     .get("type")
    //                                     .and_then(|v| v.as_str())
    //                                     .unwrap_or_default()
    //                                     .to_string(),

    //                                 context: {
    //                                     let context = message
    //                                         .get("context")
    //                                         .unwrap_or(&serde_json::Value::Null);
    //                                     MsgBtnContext {
    //                                         from: context
    //                                             .get("from")
    //                                             .and_then(|v| v.as_str())
    //                                             .unwrap_or_default()
    //                                             .to_string(),

    //                                         id: context
    //                                             .get("id")
    //                                             .and_then(|v| v.as_str())
    //                                             .unwrap_or_default()
    //                                             .to_string(),
    //                                     }
    //                                 },

    //                                 button: {
    //                                     let button_details = message
    //                                         .get("button")
    //                                         .unwrap_or(&serde_json::Value::Null);
    //                                     ButtonDetails {
    //                                         text: button_details
    //                                             .get("text")
    //                                             .and_then(|v| v.as_str())
    //                                             .unwrap_or_default()
    //                                             .to_string(),

    //                                         payload: button_details
    //                                             .get("payload")
    //                                             .and_then(|v| v.as_str())
    //                                             .unwrap_or_default()
    //                                             .to_string(),
    //                                     }
    //                                 },
    //                             };

    //                             let payload = json!(button_message);

    //                             message_payload.push(payload);
    //                         } else if r#type == "sticker" {
    //                             let sticker_message = Sticker {
    //                                 from: message
    //                                     .get("from")
    //                                     .and_then(|v| v.as_str())
    //                                     .unwrap_or_default()
    //                                     .to_string(),

    //                                 id: message
    //                                     .get("id")
    //                                     .and_then(|v| v.as_str())
    //                                     .unwrap_or_default()
    //                                     .to_string(),

    //                                 timestamp: message
    //                                     .get("timestamp")
    //                                     .and_then(|v| v.as_str())
    //                                     .unwrap_or_default()
    //                                     .to_string(),

    //                                 r#type: message
    //                                     .get("type")
    //                                     .and_then(|v| v.as_str())
    //                                     .unwrap_or_else(|| "sticker")
    //                                     .to_string(),

    //                                 sticker: {
    //                                     let sticker = message
    //                                         .get("sticker")
    //                                         .unwrap_or(&serde_json::Value::Null);
    //                                     StickerData {
    //                                         id: sticker
    //                                             .get("id")
    //                                             .and_then(|v| v.as_str())
    //                                             .unwrap_or_default()
    //                                             .to_string(),

    //                                         animated: sticker
    //                                             .get("animated")
    //                                             .and_then(|v| v.as_bool())
    //                                             .unwrap_or(false),

    //                                         mime_type: sticker
    //                                             .get("mime_type")
    //                                             .and_then(|v| v.as_str())
    //                                             .unwrap_or_default()
    //                                             .to_string(),

    //                                         sha256: sticker
    //                                             .get("sha256")
    //                                             .and_then(|v| v.as_str())
    //                                             .unwrap_or_default()
    //                                             .to_string(),
    //                                     }
    //                                 },
    //                             };

    //                             let payload = json!(sticker_message);

    //                             message_payload.push(payload);
    //                         } else if r#type == "video" {
    //                             let video_message = Video {
    //                                 from: message
    //                                     .get("from")
    //                                     .and_then(|v| v.as_str())
    //                                     .unwrap_or_default()
    //                                     .to_string(),

    //                                 id: message
    //                                     .get("id")
    //                                     .and_then(|v| v.as_str())
    //                                     .unwrap_or_default()
    //                                     .to_string(),

    //                                 timestamp: message
    //                                     .get("timestamp")
    //                                     .and_then(|v| v.as_str())
    //                                     .unwrap_or_default()
    //                                     .to_string(),

    //                                 r#type: message
    //                                     .get("type")
    //                                     .and_then(|v| v.as_str())
    //                                     .unwrap_or_else(|| "video")
    //                                     .to_string(),

    //                                 document: {
    //                                     let document = message
    //                                         .get("document")
    //                                         .unwrap_or(&serde_json::Value::Null);
    //                                     VideoData {
    //                                         caption: document
    //                                             .get("caption")
    //                                             .and_then(|v| v.as_str())
    //                                             .unwrap_or_default()
    //                                             .to_string(),

    //                                         mime_type: document
    //                                             .get("mime_type")
    //                                             .and_then(|v| v.as_str())
    //                                             .unwrap_or_default()
    //                                             .to_string(),

    //                                         sha256: document
    //                                             .get("sha256")
    //                                             .and_then(|v| v.as_str())
    //                                             .unwrap_or_default()
    //                                             .to_string(),

    //                                         id: document
    //                                             .get("id")
    //                                             .and_then(|v| v.as_str())
    //                                             .unwrap_or_default()
    //                                             .to_string(),
    //                                     }
    //                                 },
    //                             };

    //                             let payload = json!(video_message);

    //                             message_payload.push(payload);
    //                         } else if r#type == "audio" {
    //                             let audio_message = Audio {
    //                                 from: message
    //                                     .get("from")
    //                                     .and_then(|v| v.as_str())
    //                                     .unwrap_or_default()
    //                                     .to_string(),

    //                                 id: message
    //                                     .get("id")
    //                                     .and_then(|v| v.as_str())
    //                                     .unwrap_or_default()
    //                                     .to_string(),

    //                                 timestamp: message
    //                                     .get("timestamp")
    //                                     .and_then(|v| v.as_str())
    //                                     .unwrap_or_default()
    //                                     .to_string(),

    //                                 r#type: message
    //                                     .get("type")
    //                                     .and_then(|v| v.as_str())
    //                                     .unwrap_or_else(|| "audio")
    //                                     .to_string(),

    //                                 document: {
    //                                     let document = message
    //                                         .get("document")
    //                                         .unwrap_or(&serde_json::Value::Null);
    //                                     AudioData {
    //                                         caption: document
    //                                             .get("caption")
    //                                             .and_then(|v| v.as_str())
    //                                             .unwrap_or_default()
    //                                             .to_string(),

    //                                         mime_type: document
    //                                             .get("mime_type")
    //                                             .and_then(|v| v.as_str())
    //                                             .unwrap_or_default()
    //                                             .to_string(),

    //                                         sha256: document
    //                                             .get("sha256")
    //                                             .and_then(|v| v.as_str())
    //                                             .unwrap_or_default()
    //                                             .to_string(),

    //                                         id: document
    //                                             .get("id")
    //                                             .and_then(|v| v.as_str())
    //                                             .unwrap_or_default()
    //                                             .to_string(),
    //                                     }
    //                                 },
    //                             };

    //                             let payload = json!(audio_message);

    //                             message_payload.push(payload);
    //                         } else if r#type == "document" {
    //                             let document_message = Document {
    //                                 from: message
    //                                     .get("from")
    //                                     .and_then(|v| v.as_str())
    //                                     .unwrap_or_default()
    //                                     .to_string(),

    //                                 id: message
    //                                     .get("id")
    //                                     .and_then(|v| v.as_str())
    //                                     .unwrap_or_default()
    //                                     .to_string(),

    //                                 timestamp: message
    //                                     .get("timestamp")
    //                                     .and_then(|v| v.as_str())
    //                                     .unwrap_or_default()
    //                                     .to_string(),

    //                                 r#type: message
    //                                     .get("type")
    //                                     .and_then(|v| v.as_str())
    //                                     .unwrap_or_else(|| "document")
    //                                     .to_string(),

    //                                 document: {
    //                                     let document = message
    //                                         .get("document")
    //                                         .unwrap_or(&serde_json::Value::Null);
    //                                     DocumentData {
    //                                         caption: document
    //                                             .get("caption")
    //                                             .and_then(|v| v.as_str())
    //                                             .unwrap_or_default()
    //                                             .to_string(),

    //                                         mime_type: document
    //                                             .get("mime_type")
    //                                             .and_then(|v| v.as_str())
    //                                             .unwrap_or_default()
    //                                             .to_string(),

    //                                         sha256: document
    //                                             .get("sha256")
    //                                             .and_then(|v| v.as_str())
    //                                             .unwrap_or_default()
    //                                             .to_string(),

    //                                         id: document
    //                                             .get("id")
    //                                             .and_then(|v| v.as_str())
    //                                             .unwrap_or_default()
    //                                             .to_string(),
    //                                     }
    //                                 },
    //                             };

    //                             let payload = json!(document_message);

    //                             message_payload.push(payload);
    //                         } else if r#type == "image" {
    //                             let image_message = Image {
    //                                 from: message
    //                                     .get("from")
    //                                     .and_then(|v| v.as_str())
    //                                     .unwrap_or_default()
    //                                     .to_string(),

    //                                 id: message
    //                                     .get("id")
    //                                     .and_then(|v| v.as_str())
    //                                     .unwrap_or_default()
    //                                     .to_string(),

    //                                 timestamp: message
    //                                     .get("timestamp")
    //                                     .and_then(|v| v.as_str())
    //                                     .unwrap_or_default()
    //                                     .to_string(),

    //                                 r#type: message
    //                                     .get("type")
    //                                     .and_then(|v| v.as_str())
    //                                     .unwrap_or_else(|| "image")
    //                                     .to_string(),

    //                                 image: {
    //                                     let image = message
    //                                         .get("image")
    //                                         .unwrap_or(&serde_json::Value::Null);
    //                                     ImageData {
    //                                         caption: image
    //                                             .get("caption")
    //                                             .and_then(|v| v.as_str())
    //                                             .unwrap_or_default()
    //                                             .to_string(),

    //                                         mime_type: image
    //                                             .get("mime_type")
    //                                             .and_then(|v| v.as_str())
    //                                             .unwrap_or_default()
    //                                             .to_string(),

    //                                         sha256: image
    //                                             .get("sha256")
    //                                             .and_then(|v| v.as_str())
    //                                             .unwrap_or_default()
    //                                             .to_string(),

    //                                         id: image
    //                                             .get("id")
    //                                             .and_then(|v| v.as_str())
    //                                             .unwrap_or_default()
    //                                             .to_string(),
    //                                     }
    //                                 },
    //                             };

    //                             let payload = json!(image_message);

    //                             message_payload.push(payload);
    //                         } else {
    //                             return Err(format!("Message type not understood {:?}", message));
    //                         }
    //                     }
    //                 }
    //             }
    //         }
    //     }
    // }

    // Ok(message_payload)
}

pub fn find_statuses(config: Config, message: &str) -> Result<(Vec<Value>, Vec<String>), String> {
    let mut error_resp: Vec<String> = Vec::new();
    let mut success_resp: Vec<Value> = Vec::new();

    let payload: WebhookStatusesPayload = serde_json::from_str(message).unwrap();

    for (entry_index, entry) in payload.entry.iter().enumerate() {
        if entry.id.trim().trim_matches('"') != config.whatsapp_business_id {
            error_resp.push(format!("Entry id not recognized index: {}", entry_index));
            continue;
        }
        for (change_index, change) in entry.changes.iter().enumerate() {
            if change
                .value
                .metadata
                .phone_number_id
                .trim()
                .trim_matches('"')
                != config.whatsapp_phone_number_id
            {
                error_resp.push(format!(
                    "Metadata phone_number_id not recognized index: {}",
                    change_index
                ));
                continue;
            }
            for status in change.value.statuses.iter() {
                success_resp.push(json!(status));
            }
        }
    }

    Ok((success_resp, error_resp))
}
