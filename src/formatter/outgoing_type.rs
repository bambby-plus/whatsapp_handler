use serde::{Deserialize, Serialize};

/// Represents all possible message types for the WhatsApp API payload.
///
/// This enum is tagged to support various structured message types
/// such as interactive buttons, lists, media (sticker, video, audio, document, image),
/// and plain text.
#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum MessageType {
    InteractiveButton(interactive_button::InteractiveButton),
    InteractiveList(interactive_list::InteractiveList),
    Sticker(sticker::Sticker),
    Video(video::Video),
    Audio(audio::Audio),
    Document(document::Document),
    Image(image::Image),
    Text(text::Text),
}

/// Module for interactive button messages.
pub mod interactive_button {
    use super::*;

    /// Top-level structure for an interactive button message.
    #[derive(Serialize, Deserialize, Debug)]
    pub struct InteractiveButton {
        pub to: String,
        pub messaging_product: String,
        pub recipient_type: String,
        #[serde(rename = "type")]
        pub r#type: MType,
        pub interactive: Button,
    }

    /// Contains the button details.
    #[derive(Serialize, Deserialize, Debug)]
    pub struct Button {
        pub r#type: String,
        pub body: Body,
        pub action: Action,
    }

    /// The body text of the button message.
    #[derive(Serialize, Deserialize, Debug)]
    pub struct Body {
        pub text: String,
    }

    /// Contains the button actions.
    #[derive(Serialize, Deserialize, Debug)]
    pub struct Action {
        pub buttons: Vec<ReplyButton>,
    }

    /// Represents a single reply button.
    #[derive(Serialize, Deserialize, Debug)]
    pub struct ReplyButton {
        pub r#type: String,
        pub reply: ReplyDetail,
    }

    /// Details of the reply (id and title).
    #[derive(Serialize, Deserialize, Debug)]
    pub struct ReplyDetail {
        pub id: String,
        pub title: String,
    }

    /// Enum type for the message, e.g., "interactive".
    #[derive(Serialize, Deserialize, Debug)]
    #[allow(non_camel_case_types)]
    pub enum MType {
        interactive,
    }
}

/// Module for interactive list messages.
pub mod interactive_list {
    use super::*;

    /// Top-level structure for an interactive list message.
    #[derive(Serialize, Deserialize, Debug)]
    pub struct InteractiveList {
        pub to: String,
        pub messaging_product: String,
        pub recipient_type: String,
        #[serde(rename = "type")]
        pub r#type: MType,
        pub interactive: List,
    }

    /// Contains the list details.
    #[derive(Serialize, Deserialize, Debug)]
    pub struct List {
        pub r#type: String,
        pub header: Header,
        pub body: Body,
        pub footer: Footer,
        pub action: Action,
    }

    /// The header section of the list message.
    #[derive(Serialize, Deserialize, Debug)]
    pub struct Header {
        pub r#type: String,
        pub text: String,
    }

    /// The body section of the list message.
    #[derive(Serialize, Deserialize, Debug)]
    pub struct Body {
        pub text: String,
    }

    /// The footer section of the list message.
    #[derive(Serialize, Deserialize, Debug)]
    pub struct Footer {
        pub text: String,
    }

    /// Contains list actions and sections.
    #[derive(Serialize, Deserialize, Debug)]
    pub struct Action {
        pub button: String,
        pub sections: Vec<Section>,
    }

    /// Represents a section in the list.
    #[derive(Serialize, Deserialize, Debug)]
    pub struct Section {
        pub title: String,
        pub rows: Vec<Row>,
    }

    /// Represents a row inside a section.
    #[derive(Serialize, Deserialize, Debug)]
    pub struct Row {
        pub id: String,
        pub title: String,
        pub description: String,
    }

    /// Enum type for the message, e.g., "interactive".
    #[derive(Serialize, Deserialize, Debug)]
    #[allow(non_camel_case_types)]
    pub enum MType {
        interactive,
    }
}

/// Module for sticker messages.
pub mod sticker {
    use super::*;

    /// Top-level structure for a sticker message.
    #[derive(Serialize, Deserialize, Debug)]
    pub struct Sticker {
        pub to: String,
        pub messaging_product: String,
        pub recipient_type: String,
        #[serde(rename = "type")]
        pub r#type: MType,
        pub sticker: Content,
    }

    /// Content holding the sticker link.
    #[derive(Serialize, Deserialize, Debug)]
    pub struct Content {
        pub link: String,
    }

    /// Enum type for the message, e.g., "video" (likely meant to be "sticker").
    #[derive(Serialize, Deserialize, Debug)]
    #[allow(non_camel_case_types)]
    pub enum MType {
        video,
    }
}

/// Module for video messages.
pub mod video {
    use super::*;

    /// Top-level structure for a video message.
    #[derive(Serialize, Deserialize, Debug)]
    pub struct Video {
        pub to: String,
        pub messaging_product: String,
        pub recipient_type: String,
        #[serde(rename = "type")]
        pub r#type: MType,
        pub video: Content,
    }

    /// Content holding video link and caption.
    #[derive(Serialize, Deserialize, Debug)]
    pub struct Content {
        pub caption: String,
        pub link: String,
    }

    /// Enum type for the message, e.g., "video".
    #[derive(Serialize, Deserialize, Debug)]
    #[allow(non_camel_case_types)]
    pub enum MType {
        video,
    }
}

/// Module for audio messages.
pub mod audio {
    use super::*;

    /// Top-level structure for an audio message.
    #[derive(Serialize, Deserialize, Debug)]
    pub struct Audio {
        pub to: String,
        pub messaging_product: String,
        pub recipient_type: String,
        #[serde(rename = "type")]
        pub r#type: MType,
        pub audio: Content,
    }

    /// Content holding audio link.
    #[derive(Serialize, Deserialize, Debug)]
    pub struct Content {
        pub link: String,
    }

    /// Enum type for the message, e.g., "audio".
    #[derive(Serialize, Deserialize, Debug)]
    #[allow(non_camel_case_types)]
    pub enum MType {
        audio,
    }
}

/// Module for document messages.
pub mod document {
    use super::*;

    /// Top-level structure for a document message.
    #[derive(Serialize, Deserialize, Debug)]
    pub struct Document {
        pub to: String,
        pub messaging_product: String,
        pub recipient_type: String,
        #[serde(rename = "type")]
        pub r#type: MType,
        pub document: Content,
    }

    /// Content holding document link and caption.
    #[derive(Serialize, Deserialize, Debug)]
    pub struct Content {
        pub link: String,
        pub caption: String,
    }

    /// Enum type for the message, e.g., "document".
    #[derive(Serialize, Deserialize, Debug)]
    #[allow(non_camel_case_types)]
    pub enum MType {
        document,
    }
}

/// Module for image messages.
pub mod image {
    use super::*;

    /// Top-level structure for an image message.
    #[derive(Serialize, Deserialize, Debug)]
    pub struct Image {
        pub to: String,
        pub messaging_product: String,
        pub recipient_type: String,
        #[serde(rename = "type")]
        pub r#type: MType,
        pub image: Content,
    }

    /// Content holding image link.
    #[derive(Serialize, Deserialize, Debug)]
    pub struct Content {
        pub link: String,
    }

    /// Enum type for the message, e.g., "image".
    #[derive(Serialize, Deserialize, Debug)]
    #[allow(non_camel_case_types)]
    pub enum MType {
        image,
    }
}

/// Module for text messages.
pub mod text {
    use super::*;

    /// Top-level structure for a text message.
    #[derive(Serialize, Deserialize, Debug)]
    pub struct Text {
        pub to: String,
        pub messaging_product: String,
        pub recipient_type: String,
        #[serde(rename = "type")]
        pub r#type: MType,
        pub text: Content,
    }

    /// Content holding text body and preview setting.
    #[derive(Serialize, Deserialize, Debug)]
    pub struct Content {
        pub preview_url: bool,
        pub body: String,
    }

    /// Enum type for the message, e.g., "text".
    #[derive(Serialize, Deserialize, Debug)]
    #[allow(non_camel_case_types)]
    pub enum MType {
        text,
    }
}