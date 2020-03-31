use isahc::*;
use crate::StdError;
use crate::email::{Service, Email};

#[derive(Debug, serde::Deserialize)]
struct TokenResponse {
    uuid: String,
}

#[derive(Debug)]
pub struct Client {}

impl Client {
    const API_URL: &'static str = "https://webhook.site";
    const EMAIL_DOMAIN: &'static str = "email.webhook.site";

    pub fn new() -> Self { Client {} }

    fn create_token(&self) -> Result<String, StdError> {
        let response = post(format!("{}/token", Self::API_URL), "")?.json::<TokenResponse>()?;
        Ok(response.uuid)
    }
}

impl Service for Client {
    fn generate(&self) -> Result<Email, StdError> {
        let address = format!("{}@{}", self.create_token()?, Self::EMAIL_DOMAIN);
        Ok(Email::with_address(address))
    }
}
