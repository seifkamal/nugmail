pub mod error;
pub mod sqlite;

use crate::email;

pub trait Store {
    fn save_address(&mut self, address: &email::Address) -> Result<(), error::Error>;
    fn addresses(&mut self) -> Result<email::Addresses, error::Error>;
    fn delete_address(&mut self, address: &email::Address) -> Result<(), error::Error>;
    fn save_inbox(&mut self, inbox: &email::Inbox) -> Result<(), error::Error>;
    fn inbox(&mut self, address: &email::Address) -> Result<email::Inbox, error::Error>;
}
