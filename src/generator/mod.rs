pub mod webhook_site;

use crate::{email, StdError};

pub trait Service {
    fn generate(&self) -> Result<email::Address, StdError>;
    fn inbox(&self, address: &email::Address) -> Result<email::Inbox, StdError>;
    fn delete(&self, address: &email::Address) -> Result<(), StdError>;
}
