use std::fmt;

#[derive(Debug)]
pub struct Address(String);

impl Address {
    pub fn new(address: &str) -> Self { Address(address.to_string()) }
    pub fn as_str(&self) -> &String { &self.0 }
}

impl From<&str> for Address {
    fn from(address: &str) -> Self {
        Address::new(address)
    }
}

impl fmt::Display for Address {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Debug)]
pub struct Message {
    sender: Address,
    recipient: Address,
    subject: Option<String>,
    body: Option<String>,
    date: String,
}

impl Message {
    pub fn new(
        sender: Address,
        recipient: Address,
        subject: Option<String>,
        body: Option<String>,
        date: String,
    ) -> Self {
        Message { sender, recipient, subject, body, date }
    }
    pub fn sender(&self) -> &Address { &self.sender }
    pub fn subject(&self) -> Option<&String> { self.subject.as_ref() }
}

impl fmt::Display for Message {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Date: {}\nFrom: {}\nSubject: {}\n\n{}",
            self.date,
            self.sender,
            self.subject.as_ref().unwrap_or(&"<no subject>".to_string()),
            self.body.as_ref().unwrap_or(&"<no body>".to_string())
        )
    }
}

pub type Messages = Vec<Message>;

#[derive(Debug)]
pub struct Inbox {
    owner: Address,
    messages: Messages,
}

impl Inbox {
    pub fn new(owner: Address, messages: Messages) -> Self { Inbox { owner, messages } }
    pub fn messages(&self) -> &Messages { &self.messages }
    pub fn size(&self) -> usize { self.messages.len() }
}
