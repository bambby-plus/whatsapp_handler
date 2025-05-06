use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Module for handling WhatsApp **Order** messages.
pub mod order {
    use super::*;

    /// Represents an incoming order message.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Order {
        pub from: String,
        pub id: String,
        pub order: OrderDetails,
        pub context: MessageContext,
        pub timestamp: String,
        #[serde(rename = "type")]
        pub r#type: String,
    }

    /// Details of the order (catalog and products).
    #[derive(Debug, Serialize, Deserialize)]
    pub struct OrderDetails {
        pub catalog_id: String,
        pub product_items: Vec<ProductItem>,
        pub text: String,
    }

    /// Represents a single product item in the order.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct ProductItem {
        pub product_retailer_id: String,
        pub quantity: String,
        pub item_price: String,
        pub currency: String,
    }

    /// Context of the related message.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct MessageContext {
        pub from: String,
        pub id: String,
    }
}

/// Module for handling **Product Enquiry** messages.
pub mod enquiry {
    use super::*;

    /// Represents an incoming enquiry message.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Enquiry {
        pub from: String,
        pub id: String,
        pub text: EnquiryPayload,
        pub context: EnquiryContext,
        pub timestamp: String,
        #[serde(rename = "type")]
        pub r#type: String,
    }

    /// Payload containing the enquiry body.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct EnquiryPayload {
        pub body: String,
    }

    /// Context of the enquiry including the referred product.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct EnquiryContext {
        pub from: String,
        pub id: String,
        pub referred_product: ReferredProduct,
    }

    /// Details of the product the enquiry refers to.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct ReferredProduct {
        pub catalog_id: String,
        pub product_retailer_id: String,
    }
}

/// Module for handling **Unknown** or error-type messages.
pub mod unknown {
    use super::*;

    /// Represents an unknown or error message.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Unknown {
        pub from: String,
        pub id: String,
        pub timestamp: String,
        pub errors: Vec<ErrorDetail>,
        #[serde(rename = "type")]
        pub r#type: String,
    }

    /// Details about the error.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct ErrorDetail {
        pub code: u32,
        pub details: String,
        pub title: String,
    }
}

/// Module for handling **Ads Referral** messages.
pub mod ads {
    use super::*;

    /// Represents a referral ad message.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Ads {
        pub referral: Referral,
        pub from: String,
        pub id: String,
        pub timestamp: String,
        #[serde(rename = "type")]
        pub r#type: String,
        pub text: TextBody,
    }

    /// Details about the referral source.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Referral {
        pub source_url: String,
        pub source_id: String,
        pub source_type: String,
        pub headline: String,
        pub body: String,
        pub media_type: String,
        pub image_url: String,
        pub video_url: String,
        pub thumbnail_url: String,
    }

    /// Text body of the ad.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct TextBody {
        pub body: String,
    }
}

/// Module for handling **Location** messages.
pub mod location {
    use super::*;

    /// Represents a shared location message.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Location {
        pub from: String,
        pub id: String,
        pub timestamp: String,
        pub location: LocationPayload,
    }

    /// Details of the location shared.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct LocationPayload {
        pub latitude: String,
        pub longitude: String,
        pub name: String,
        pub address: String,
    }
}

/// Module for handling **Contact** messages.
pub mod contact {
    use super::*;

    /// Represents a contact card message.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Contact {
        pub addresses: Vec<Address>,
        pub birthday: String,
        pub emails: Vec<Email>,
        pub name: Name,
        pub org: Organization,
        pub phones: Vec<Phone>,
        pub urls: Vec<Url>,
    }

    /// Address details of the contact.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Address {
        pub city: String,
        pub country: String,
        pub country_code: String,
        pub state: String,
        pub street: String,
        #[serde(rename = "type")]
        pub r#type: String,
        pub zip: String,
    }

    /// Email details.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Email {
        pub email: String,
        #[serde(rename = "type")]
        pub r#type: String,
    }

    /// Name details.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Name {
        pub formatted_name: String,
        pub first_name: String,
        pub last_name: String,
        pub middle_name: String,
        pub suffix: String,
        pub prefix: String,
    }

    /// Organization details.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Organization {
        pub company: String,
        pub department: String,
        pub title: String,
    }

    /// Phone details.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Phone {
        pub phone: String,
        pub wa_id: String,
        #[serde(rename = "type")]
        pub r#type: String,
    }

    /// URL details.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Url {
        pub url: String,
        #[serde(rename = "type")]
        pub r#type: String,
    }
}

/// Module for handling **Reaction** messages.
pub mod reaction {
    use super::*;

    /// Represents a reaction to a message (like emoji).
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Reaction {
        pub from: String,
        pub id: String,
        pub timestamp: String,
        pub r#type: String,
        pub reaction: ReactionPayload,
    }

    /// Details of the reaction.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct ReactionPayload {
        pub emoji: String,
        pub message_id: String,
    }
}

/// Module for handling **Button** reply messages.
pub mod button {
    use super::*;

    /// Represents a button click message.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Button {
        pub context: MsgBtnContext,
        pub from: String,
        pub id: String,
        pub timestamp: String,
        #[serde(rename = "type")]
        pub r#type: String,
        pub button: ButtonDetails,
    }

    /// Context of the button message.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct MsgBtnContext {
        pub from: String,
        pub id: String,
    }

    /// Details of the button clicked.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct ButtonDetails {
        pub text: String,
        pub payload: String,
    }
}

/// Module for handling **Sticker** messages.
pub mod sticker {
    use super::*;

    /// Represents a sticker message.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Sticker {
        pub from: String,
        pub id: String,
        pub timestamp: String,
        pub r#type: String, // escape Rust keyword 'type'
        pub sticker: StickerData,
    }

    /// Details of the sticker.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct StickerData {
        pub id: String,
        pub animated: bool,
        pub mime_type: String,
        pub sha256: String,
    }
}

/// Module for handling **Video** messages.
pub mod video {
    use super::*;

    /// Represents a video message.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Video {
        pub from: String,
        pub id: String,
        pub timestamp: String,
        pub r#type: String,
        pub document: VideoData,
    }

    /// Details of the video.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct VideoData {
        pub caption: String,
        pub mime_type: String,
        pub sha256: String,
        pub id: String,
    }
}

/// Module for handling **Audio** messages.
pub mod audio {
    use super::*;

    /// Represents an audio message.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Audio {
        pub from: String,
        pub id: String,
        pub timestamp: String,
        pub r#type: String,
        pub document: AudioData,
    }

    /// Details of the audio file.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct AudioData {
        pub caption: String,
        pub mime_type: String,
        pub sha256: String,
        pub id: String,
    }
}

/// Module for handling **Document** messages.
pub mod document {
    use super::*;

    /// Represents a document message.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Document {
        pub from: String,
        pub id: String,
        pub timestamp: String,
        pub r#type: String,
        pub document: DocumentData,
    }

    /// Details of the document.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct DocumentData {
        pub caption: String,
        pub mime_type: String,
        pub sha256: String,
        pub id: String,
    }
}

/// Module for handling **Image** messages.
pub mod image {
    use super::*;

    /// Represents an image message.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Image {
        pub from: String,
        pub id: String,
        pub timestamp: String,
        pub r#type: String,
        pub image: ImageData,
    }

    /// Details of the image.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct ImageData {
        pub caption: String,
        pub mime_type: String,
        pub sha256: String,
        pub id: String,
    }
}

/// Module for handling **Text** messages.
pub mod text {
    use super::*;

    /// Represents a text message.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Text {
        pub from: String,
        pub id: String,
        pub timestamp: String,
        pub r#type: String, // use `r#type` because `type` is a reserved Rust keyword
        pub text: TextPayload,
    }

    /// Details of the text body.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct TextPayload {
        pub body: String,
    }
}

/// Module for individual status updates in the WhatsApp API.
pub mod statuses {
    use super::*;

    /// Represents a status update event.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct StatusUpdate {
        /// Unique ID of the status.
        pub id: String,
        /// Optional recipient ID.
        pub recipient_id: Option<String>,
        /// Status string (e.g., delivered, read).
        pub status: String,
        /// Timestamp of the event.
        pub timestamp: String,
        /// Optional conversation details.
        pub conversation: Option<Conversation>,
        /// Optional pricing information.
        pub pricing: Option<Pricing>,
    }

    /// Details about the conversation.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Conversation {
        /// Conversation ID.
        pub id: String,
        /// Expiration timestamp for the conversation.
        pub expiration_timestamp: String,
        /// Origin details.
        pub origin: Origin,
    }

    /// Represents the origin of the conversation.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Origin {
        /// Type of origin (user_initiated, business_initiated, referral_conversion).
        #[serde(rename = "type")]
        pub origin_type: String,
    }

    /// Pricing details for the message.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Pricing {
        /// Pricing model (e.g., CBP).
        pub pricing_model: String,
        /// Whether the message is billable.
        pub billable: bool,
        /// Category of the message (user_initiated, business_initiated, referral_conversion).
        pub category: String,
    }
}

/// Module representing the full webhook payload for statuses.
pub mod full_statuses_payload {
    use super::*;

    /// Top-level webhook payload for status events.
    #[derive(Debug, Deserialize, Serialize)]
    pub struct WebhookStatusesPayload {
        /// Object type (usually "whatsapp_business_account").
        pub object: String,
        /// List of entries in the payload.
        pub entry: Vec<Entry>,
    }

    /// An individual entry in the payload.
    #[derive(Debug, Deserialize, Serialize)]
    pub struct Entry {
        /// Entry ID.
        pub id: String,
        /// List of changes inside the entry.
        pub changes: Vec<Change>,
    }

    /// Represents a change event in the entry.
    #[derive(Debug, Deserialize, Serialize)]
    pub struct Change {
        /// Change value details.
        pub value: Vaalue,
        /// Field name (usually "statuses").
        pub field: String,
    }

    /// Details of the change value.
    #[derive(Debug, Deserialize, Serialize)]
    pub struct Vaalue {
        /// Messaging product type (usually "whatsapp").
        pub messaging_product: String,
        /// Metadata details.
        pub metadata: Metadata,
        /// List of status updates.
        pub statuses: Vec<Value>,
    }

    /// Metadata about the phone number.
    #[derive(Debug, Deserialize, Serialize)]
    pub struct Metadata {
        /// Displayed phone number.
        pub display_phone_number: String,
        /// Internal phone number ID.
        pub phone_number_id: String,
    }
}

/// Module representing the full webhook payload for incoming messages.
pub mod full_messages_payload {
    use super::*;

    /// Top-level webhook payload for message events.
    #[derive(Debug, Deserialize, Serialize)]
    pub struct WebhookMessagePayload {
        /// Object type (usually "whatsapp_business_account").
        pub object: String,
        /// List of entries in the payload.
        pub entry: Vec<Entry>,
    }

    /// An individual entry in the payload.
    #[derive(Debug, Deserialize, Serialize)]
    pub struct Entry {
        /// Entry ID.
        pub id: String,
        /// List of changes inside the entry.
        pub changes: Vec<Change>,
    }

    /// Represents a change event in the entry.
    #[derive(Debug, Deserialize, Serialize)]
    pub struct Change {
        /// Change value details.
        pub value: Vaalue,
        /// Field name (usually "messages").
        pub field: String,
    }

    /// Details of the change value.
    #[derive(Debug, Deserialize, Serialize)]
    pub struct Vaalue {
        /// Messaging product type (usually "whatsapp").
        pub messaging_product: String,
        /// Metadata details.
        pub metadata: Metadata,
        /// List of contacts involved.
        pub contacts: Vec<Contact>,
        /// List of message events.
        pub messages: Vec<Value>,
    }

    /// Metadata about the phone number.
    #[derive(Debug, Deserialize, Serialize)]
    pub struct Metadata {
        /// Displayed phone number.
        pub display_phone_number: String,
        /// Internal phone number ID.
        pub phone_number_id: String,
    }

    /// Contact details for the message.
    #[derive(Debug, Deserialize, Serialize)]
    pub struct Contact {
        /// Profile information.
        pub profile: Profile,
        /// WhatsApp ID of the contact.
        pub wa_id: String,
    }

    /// User profile details.
    #[derive(Debug, Deserialize, Serialize)]
    pub struct Profile {
        /// Contact's name.
        pub name: String,
    }
}
