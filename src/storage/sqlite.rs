use crate::StdError;
use crate::email::{Address, Inbox, Messages, Message, Addresses};
use crate::storage::{Store, Error};
use rusqlite::{Connection, Error as RusqliteError, Row, ToSql, NO_PARAMS};
use rusqlite::types::{FromSql, FromSqlResult, ValueRef, ToSqlOutput};

pub fn new_connection(file_path: &str) -> Result<Connection, StdError> {
    let connection = Connection::open(file_path)?;
    connection.pragma_update(None, "foreign_keys", &"on")?;
    Ok(connection)
}

pub struct Storage {
    connection: Connection,
}

impl Storage {
    pub fn new(connection: Connection) -> Self {
        Storage { connection }
    }
}

impl Store for Storage {
    fn save_address(&mut self, address: &Address) -> Result<(), Error> {
        self.connection.execute_named(
            "INSERT OR IGNORE INTO email_addresses (address) VALUES (:address)",
            &[(":address", address)]
        )?;
        Ok(())
    }

    fn addresses(&mut self) -> Result<Addresses, Error> {
        let mut stmt = self.connection.prepare("SELECT * FROM email_addresses")?;
        let mut rows = stmt.query(NO_PARAMS)?;

        let mut addresses = Addresses::new();
        while let Some(row) = rows.next()? {
            addresses.push(row.get_unwrap::<_, Address>(1))
        }

        Ok(addresses)
    }

    fn delete_address(&mut self, address: &Address) -> Result<(), Error> {
        self.connection.execute_named(
            "DELETE FROM email_addresses WHERE address=:address",
            &[(":address", address)]
        )?;
        Ok(())
    }

    fn save_inbox(&mut self, inbox: &Inbox) -> Result<(), Error> {
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
                ]
            )?;
        }

        Ok(())
    }

    fn inbox(&mut self, address: &Address) -> Result<Inbox, Error> {
        let mut stmt = self.connection.prepare("SELECT * FROM emails WHERE recipient=:address")?;
        let rows = stmt.query_map_named::<Message, _>(
            &[(":address", address)],
            |row| Ok(Message::from(row)),
        )?;

        let mut messages = Messages::new();
        for row in rows.into_iter() {
            messages.push(row?);
        }

        Ok(Inbox::new(address.clone(), messages))
    }
}

impl Default for Storage {
    fn default() -> Self {
        Storage::new(new_connection("nugmail.db").unwrap())
    }
}

impl FromSql for Address {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        Ok(Address::from(value.as_str()?))
    }
}

impl ToSql for Address {
    fn to_sql(&self) -> Result<ToSqlOutput<'_>, RusqliteError> {
        Ok(ToSqlOutput::Borrowed(ValueRef::Text(self.as_str().as_ref())))
    }
}

impl From<&Row<'_>> for Message {
    fn from(row: &Row<'_>) -> Self {
        Message::new(
            row.get_unwrap(1),
            row.get_unwrap::<_, Address>(2),
            row.get_unwrap::<_, Address>(3),
            Some(row.get_unwrap(4)),
            Some(row.get_unwrap(5)),
            row.get_unwrap(6),
        )
    }
}

impl From<RusqliteError> for Error {
    fn from(error: RusqliteError) -> Self {
        match error {
            RusqliteError::QueryReturnedNoRows => Error::NotFound,
            _ => Error::OperationFailed(Box::new(error)),
        }
    }
}
