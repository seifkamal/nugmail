use crate::StdError;
use crate::email::{Address, Inbox};

pub trait Service {
    fn generate(&self) -> Result<Address, StdError>;
    fn inbox(&self, address: &Address) -> Result<Inbox, StdError>;
}

pub mod webhook_site;
