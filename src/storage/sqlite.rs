use rusqlite::types;

use crate::{
    StdError,
    email,
    storage::{Store, error::Error},
};

pub fn new_connection(file_path: &str) -> Result<rusqlite::Connection, StdError> {
    let connection = rusqlite::Connection::open(file_path)?;
    connection.pragma_update(None, "foreign_keys", &"on")?;
    Ok(connection)
}

pub struct Storage {
    connection: rusqlite::Connection,
}

impl Storage {
    pub fn new(connection: rusqlite::Connection) -> Self {
        Storage { connection }
    }
}

impl Store for Storage {
    fn save_address(&mut self, address: &email::Address) -> Result<(), Error> {
        self.connection.execute_named(
            "INSERT OR IGNORE INTO email_addresses (address) VALUES (:address)",
            &[(":address", address)],
        )?;
        Ok(())
    }

    fn addresses(&mut self) -> Result<email::Addresses, Error> {
        let mut stmt = self.connection.prepare("SELECT * FROM email_addresses")?;
        let mut rows = stmt.query(rusqlite::NO_PARAMS)?;

        let mut addresses = email::Addresses::new();
        while let Some(row) = rows.next()? {
            addresses.push(row.get_unwrap::<_, email::Address>(1))
        }

        Ok(addresses)
    }


    fn delete_address(&mut self, address: &email::Address) -> Result<(), Error> {
        self.connection.execute_named(
            "DELETE FROM email_addresses WHERE address=:address",
            &[(":address", address)],
        )?;
        Ok(())
    }

    fn save_inbox(&mut self, inbox: &email::Inbox) -> Result<(), Error> {
        for message in inbox.messages().iter() {
            self.connection.execute_named(
                "INSERT OR IGNORE INTO emails (remote_id, sender, recipient, subject, body, received_at) VALUES (:remote_id, :sender, :recipient, :subject, :body, :received_at)",
                &[
                    (":remote_id", message.remote_id()),
                    (":sender", message.sender()),
                    (":recipient", message.recipient()),
                    (":subject", &message.subject()),
                    (":body", &message.body()),
                    (":received_at", message.received_at()),
                ],
            )?;
        }

        Ok(())
    }

    fn inbox(&mut self, address: &email::Address) -> Result<email::Inbox, Error> {
        let mut stmt = self.connection.prepare("SELECT * FROM emails WHERE recipient=:address")?;
        let rows = stmt.query_map_named::<email::Message, _>(
            &[(":address", address)],
            |row| Ok(email::Message::from(row)),
        )?;

        let mut messages = email::Messages::new();
        for row in rows.into_iter() {
            messages.push(row?);
        }

        Ok(email::Inbox::new(address.clone(), messages))
    }
}

impl Default for Storage {
    fn default() -> Self {
        Storage::new(new_connection("nugmail.db").unwrap())
    }
}

impl types::FromSql for email::Address {
    fn column_result(value: types::ValueRef<'_>) -> types::FromSqlResult<Self> {
        Ok(email::Address::from(value.as_str()?))
    }
}

impl rusqlite::ToSql for email::Address {
    fn to_sql(&self) -> Result<types::ToSqlOutput<'_>, rusqlite::Error> {
        Ok(types::ToSqlOutput::Borrowed(types::ValueRef::Text(self.as_str().as_ref())))
    }
}

impl From<&rusqlite::Row<'_>> for email::Message {
    fn from(row: &rusqlite::Row<'_>) -> Self {
        email::Message::new(
            row.get_unwrap(1),
            row.get_unwrap::<_, email::Address>(2),
            row.get_unwrap::<_, email::Address>(3),
            Some(row.get_unwrap(4)),
            Some(row.get_unwrap(5)),
            row.get_unwrap(6),
        )
    }
}

impl From<rusqlite::Error> for Error {
    fn from(error: rusqlite::Error) -> Self {
        match error {
            rusqlite::Error::QueryReturnedNoRows => Error::NotFound,
            _ => Error::OperationFailed(Box::new(error)),
        }
    }
}
