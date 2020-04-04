use crate::StdError;
use crate::email::{Address, Inbox};

pub trait Service {
    fn generate(&self) -> Result<Address, StdError>;
    fn inbox(&self, address: &Address) -> Result<Inbox, StdError>;
    fn delete(&self, address: &Address) -> Result<(), StdError>;
}

pub mod webhook_site;
