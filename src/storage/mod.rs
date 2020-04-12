pub mod error;
pub mod sqlite;

use crate::email;

pub trait Store {
    fn address(&mut self, address: &str) -> Result<email::Address, error::Error>;
    fn save_address(&mut self, address: &email::Address) -> Result<(), error::Error>;
    fn delete_address(&mut self, address: &email::Address) -> Result<(), error::Error>;
    fn addresses(&mut self) -> Result<email::Addresses, error::Error>;
    fn inbox(&mut self, address: &email::Address) -> Result<email::Inbox, error::Error>;
    fn save_inbox(&mut self, inbox: &email::Inbox) -> Result<(), error::Error>;
}
