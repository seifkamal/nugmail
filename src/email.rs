use std::fmt::{Display, Formatter, Result as fmtResult};
use crate::StdError;

#[derive(Debug)]
pub struct Email {
    address: String,
}

impl Email {
    pub fn with_address(address: String) -> Self { Email { address } }
}

impl Display for Email {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmtResult {
        f.write_str(&self.address)
    }
}

pub trait Service {
    fn generate(&self) -> Result<Email, StdError>;
}
