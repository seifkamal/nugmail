use crate::StdError;
use crate::email::{Address, Inbox, Messages, Message};
use crate::generator::Service;
use isahc::*;
use std::collections::HashMap;

trait Token {
    fn token(&self) -> &str;
}

impl Token for Address {
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
    text_content: String,
    created_at: String,
    headers: HashMap<String, Vec<String>>
}

#[derive(Debug, serde::Deserialize)]
struct MessagesResponse {
    data: Vec<MessagesResponseItem>
}

impl Service for Client {
    fn generate(&self) -> Result<Address, StdError> {
        let response = post(format!("{}/token", Self::API_URL), "")?.json::<TokenResponse>()?;
        let address = format!("{}@{}", response.uuid, Self::EMAIL_DOMAIN);
        Ok(Address::from(address.as_str()))
    }

    fn inbox(&self, address: &Address) -> Result<Inbox, StdError> {
        let response = get(format!(
            "{}/token/{}/requests",
            Client::API_URL,
            address.token()
        ))?.json::<MessagesResponse>()?;

        let mut messages = Messages::new();
        for item in response.data.iter() {
            messages.push(Message::new(
                item.uuid.clone(),
                Address::from(item.sender.as_str()),
                address.clone(),
                item.headers["subject"].get(0).cloned(),
                Some(item.text_content.clone()),
                item.created_at.clone()
            ));
        }

        Ok(Inbox::new(address.clone(), messages))
    }

    fn delete(&self, address: &Address) -> Result<(), StdError> {
        delete(format!(
            "{}/token/{}",
            Client::API_URL,
            address.token()
        ))?;

        Ok(())
    }
}
