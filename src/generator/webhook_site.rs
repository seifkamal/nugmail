use isahc::ResponseExt;
use std::collections::HashMap;

use crate::{
    email,
    generator::Service,
    StdError,
};

trait Token {
    fn token(&self) -> &str;
}

impl Token for email::Address {
    fn token(&self) -> &str {
        self.as_str()[..36].as_ref()
    }
}

#[derive(Debug)]
pub struct Client {}

impl Client {
    const API_URL: &'static str = "https://webhook.site";
    const EMAIL_DOMAIN: &'static str = "email.webhook.site";

    pub fn new() -> Self { Client {} }
}

#[derive(Debug, serde::Deserialize)]
struct TokenResponse {
    uuid: String,
}

#[derive(Debug, serde::Deserialize)]
struct MessagesResponseItem {
    uuid: String,
    sender: String,
    text_content: Option<String>,
    created_at: String,
    headers: HashMap<String, Vec<String>>,
}

#[derive(Debug, serde::Deserialize)]
struct MessagesResponse {
    data: Vec<MessagesResponseItem>
}

impl Service for Client {
    fn generate(&self) -> Result<email::Address, StdError> {
        let response = isahc::post(format!("{}/token", Self::API_URL), "")?.json::<TokenResponse>()?;
        let address = format!("{}@{}", response.uuid, Self::EMAIL_DOMAIN);
        Ok(email::Address::from(address.as_str()))
    }

    fn inbox(&self, address: &email::Address) -> Result<email::Inbox, StdError> {
        let response = isahc::get(format!(
            "{}/token/{}/requests",
            Client::API_URL,
            address.token()
        ))?.json::<MessagesResponse>()?;

        let mut messages = email::Messages::new();
        for item in response.data.iter() {
            messages.push(email::Message::new(
                item.uuid.clone(),
                email::Address::from(item.sender.as_str()),
                address.clone(),
                item.headers["subject"].get(0).cloned(),
                item.text_content.clone(),
                item.created_at.clone(),
            ));
        }

        Ok(email::Inbox::new(address.clone(), messages))
    }

    fn delete(&self, address: &email::Address) -> Result<(), StdError> {
        isahc::delete(format!(
            "{}/token/{}",
            Client::API_URL,
            address.token()
        ))?;

        Ok(())
    }
}
