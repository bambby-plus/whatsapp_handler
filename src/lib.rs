pub mod action;
pub mod config;
pub mod formatter;

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[ignore] // remove ignore incase you want to test
    async fn incoming_message() {
        let whatsapp_base_url = String::from("https://graph.facebook.com");
        let whatsapp_version = String::from("v17.0");
        let whatsapp_business_id = String::from("");
        let whatsapp_phone_number_id = String::from("");
        let whatsapp_system_user_token = String::from("");

        let incoming_stringify = r#"
{
    "object": "whatsapp_business_account",
    "entry": [
        {
            "id": "whatsapp_business_id",
            "changes": [
                {
                    "value": {
                        "messaging_product": "whatsapp",
                        "metadata": {
                            "display_phone_number": "PHONE_NUMBER",
                            "phone_number_id": "whatsapp_phone_number_id"
                        },
                        "contacts": [
                            {
                                "profile": {
                                    "name": ""
                                },
                                "wa_id": ""
                            }
                        ],
                        "messages": [
                            {
                                "from": "",
                                "id": "wamid",
                                "timestamp": "",
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

        // let messages = formatter::incoming::from(config, incoming_stringify);
        let messages = config::Config::from(
            whatsapp_base_url,
            whatsapp_version,
            whatsapp_business_id,
            whatsapp_phone_number_id,
            whatsapp_system_user_token,
        )
        .incoming_message(incoming_stringify);
        println!("Incoming message: {:?}", messages)
    }

    #[tokio::test]
    #[ignore] // remove ignore incase you want to test
    async fn incoming_statuses() {
        let whatsapp_base_url = String::from("https://graph.facebook.com");
        let whatsapp_version = String::from("v17.0");
        let whatsapp_business_id = String::from("");
        let whatsapp_phone_number_id = String::from("");
        let whatsapp_system_user_token = String::from("");

        let incoming_stringify = r#"
{
        "object": "whatsapp_business_account",
        "entry": [
            {
                "id": "whatsapp_business_id",
                "changes": [
                    {
                        "value": {
                            "messaging_product": "whatsapp",
                            "metadata": {
                                "display_phone_number": "PHONE_NUMBER",
                                "phone_number_id": "whatsapp_phone_number_id"
                            },
                            "statuses": [
                                {
                                    "id": "wamid.ID",
                                    "status": "sent",
                                    "timestamp": "TIMESTAMP",
                                    "recipient_id": "PHONE_NUMBER",
                                    "conversation": {
                                        "id": "CONVERSATION_ID",
                                        "expiration_timestamp": "TIMESTAMP",
                                        "origin": {
                                            "type": "referral_conversion"
                                        }
                                    },
                                    "pricing": {
                                        "billable": false,
                                        "pricing_model": "CBP",
                                        "category": "referral_conversion"
                                    }
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

        // let messages = formatter::incoming::from(config, incoming_stringify);
        let messages = config::Config::from(
            whatsapp_base_url,
            whatsapp_version,
            whatsapp_business_id,
            whatsapp_phone_number_id,
            whatsapp_system_user_token,
        )
        .incoming_statuses(incoming_stringify);
        println!("Incoming Status: {:?}", messages)
    }

    #[tokio::test]
    #[ignore] // remove ignore incase you want to test
    async fn outgoing_text_message() {
        use formatter::outgoing_type::MessageType;
        use formatter::outgoing_type::text::{Content, MType, Text};

        let whatsapp_base_url = String::from("https://graph.facebook.com");
        let whatsapp_version = String::from("v17.0");
        let whatsapp_business_id = String::from("");
        let whatsapp_phone_number_id = String::from("");
        let whatsapp_system_user_token = String::from("");

        let message = Text {
            to: String::from(""),
            messaging_product: String::from("whatsapp"),
            recipient_type: String::from("individual"),
            r#type: MType::text,
            text: Content {
                preview_url: false,
                body: String::from("Hello from Rust library, Sweet!!"),
            },
        };

        // let message = formatter::outgoing::from(config, MessageType::Text(message)).await;
        let response = config::Config::from(
            whatsapp_base_url,
            whatsapp_version,
            whatsapp_business_id,
            whatsapp_phone_number_id,
            whatsapp_system_user_token,
        )
        .outgoing(MessageType::Text(message))
        .await;
        println!("Outgoing message: {:?}", response)
    }

    #[tokio::test]
    #[ignore] // remove ignore incase you want to test
    async fn outgoing_interaction_list_message() {
        use formatter::outgoing_type::MessageType;
        use formatter::outgoing_type::interactive_list::{
            Action, Body, Footer, Header, InteractiveList, List, MType, Row, Section,
        };

        let whatsapp_base_url = String::from("https://graph.facebook.com");
        let whatsapp_version = String::from("v17.0");
        let whatsapp_business_id = String::from("");
        let whatsapp_phone_number_id = String::from("");
        let whatsapp_system_user_token = String::from("");

        let message = InteractiveList {
            to: "".to_string(),
            messaging_product: "whatsapp".to_string(),
            recipient_type: "individual".to_string(),
            r#type: MType::interactive,
            interactive: List {
                r#type: "list".to_string(),
                header: Header {
                    r#type: "text".to_string(),
                    text: "Header Text".to_string(),
                },
                body: Body {
                    text: "This is the body text.".to_string(),
                },
                footer: Footer {
                    text: "This is the footer text.".to_string(),
                },
                action: Action {
                    button: "Choose an option".to_string(),
                    sections: vec![
                        Section {
                            title: "Section 1".to_string(),
                            rows: vec![
                                Row {
                                    id: "row1".to_string(),
                                    title: "Row 1 Title".to_string(),
                                    description: "Description for Row 1".to_string(),
                                },
                                Row {
                                    id: "row2".to_string(),
                                    title: "Row 2 Title".to_string(),
                                    description: "Description for Row 2".to_string(),
                                },
                            ],
                        },
                        Section {
                            title: "Section 2".to_string(),
                            rows: vec![Row {
                                id: "row3".to_string(),
                                title: "Row 3 Title".to_string(),
                                description: "Description for Row 3".to_string(),
                            }],
                        },
                    ],
                },
            },
        };

        // let message = formatter::outgoing::from(config, MessageType::Text(message)).await;
        let response = config::Config::from(
            whatsapp_base_url,
            whatsapp_version,
            whatsapp_business_id,
            whatsapp_phone_number_id,
            whatsapp_system_user_token,
        )
        .outgoing(MessageType::InteractiveList(message))
        .await;
        println!("Outgoing interaction message: {:?}", response)
    }

    #[tokio::test]
    #[ignore] // remove ignore incase you want to test
    async fn outgoing_template_otp_verification() {
        use formatter::outgoing_type::MessageType;
        use formatter::outgoing_type::template::{
            Component, Language, MType, Parameter, Template, TemplateContent, TextParameter,
        };

        let whatsapp_base_url = String::from("https://graph.facebook.com");
        let whatsapp_version = String::from("v22.0");
        let whatsapp_business_id = String::from("");
        let whatsapp_phone_number_id = String::from("");
        let whatsapp_system_user_token = String::from("");

        let message = Template {
            messaging_product: "whatsapp".to_string(),
            recipient_type: "individual".to_string(),
            to: "2349066332543".to_string(),
            r#type: MType::template,
            template: TemplateContent {
                name: "otp_verification_2".to_string(),
                language: Language {
                    code: "en".to_string(),
                },
                components: vec![
                    // Body component with text parameter
                    Component {
                        r#type: "body".to_string(),
                        parameters: Some(vec![Parameter::Text(TextParameter {
                            r#type: "text".to_string(),
                            text: "12345".to_string(),
                        })]),
                        sub_type: None,
                        index: None,
                    },
                    // Button component with URL type
                    Component {
                        r#type: "button".to_string(),
                        parameters: Some(vec![Parameter::Text(TextParameter {
                            r#type: "text".to_string(),
                            text: "12345".to_string(),
                        })]),
                        sub_type: Some("url".to_string()),
                        index: Some("0".to_string()),
                    },
                ],
            },
        };

        let response = config::Config::from(
            whatsapp_base_url,
            whatsapp_version,
            whatsapp_business_id,
            whatsapp_phone_number_id,
            whatsapp_system_user_token,
        )
        .outgoing(MessageType::Template(message))
        .await;
        println!("Outgoing template message: {:?}", response)
    }
}
