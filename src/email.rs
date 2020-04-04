use std::fmt;

#[derive(Debug)]
pub struct Address(String);

impl Address {
    pub fn new(address: &str) -> Self {
        Address(address.to_string())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Clone for Address {
    fn clone(&self) -> Self {
        Address::new(self.as_str())
    }

    fn clone_from(&mut self, source: &Self) {
        self.0 = source.as_str().to_string()
    }
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

pub type Addresses = Vec<Address>;

#[derive(Debug)]
pub struct Message {
    remote_id: String,
    sender: Address,
    recipient: Address,
    subject: Option<String>,
    body: Option<String>,
    received_at: String,
}

impl Message {
    pub fn new(
        remote_id: String,
        sender: Address,
        recipient: Address,
        subject: Option<String>,
        body: Option<String>,
        received_at: String,
    ) -> Self {
        Message { remote_id, sender, recipient, subject, body, received_at }
    }

    pub fn remote_id(&self) -> &String {
        &self.remote_id
    }

    pub fn sender(&self) -> &Address {
        &self.sender
    }

    pub fn recipient(&self) -> &Address {
        &self.recipient
    }

    pub fn subject(&self) -> Option<&String> {
        self.subject.as_ref()
    }

    pub fn body(&self) -> Option<&String> {
        self.body.as_ref()
    }

    pub fn received_at(&self) -> &String {
        &self.received_at
    }
}

impl fmt::Display for Message {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Date: {}\nFrom: {}\nSubject: {}\n\n{}",
            self.received_at,
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
    pub fn new(owner: Address, messages: Messages) -> Self {
        Inbox { owner, messages }
    }

    pub fn messages(&self) -> &Messages {
        &self.messages
    }

    pub fn size(&self) -> usize {
        self.messages.len()
    }
}
