use crate::StdError;
use crate::email::Address;

pub trait Service {
    fn generate(&self) -> Result<Address, StdError>;
}

pub mod webhook_site;
