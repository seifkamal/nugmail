use crate::StdError;
use crate::email::Address;
use crate::generator::Service;
use isahc::*;

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
}

impl Service for Client {
    fn generate(&self) -> Result<Address, StdError> {
        let response = post(format!("{}/token", Self::API_URL), "")?.json::<TokenResponse>()?;
        let address = format!("{}@{}", response.uuid, Self::EMAIL_DOMAIN);
        Ok(Address::new(&address))
    }
}
