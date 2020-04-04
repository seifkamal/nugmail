use crate::StdError;
use crate::email::{Address, Inbox, Messages, Message, Addresses};
use crate::storage::{Store, Error};
use rusqlite::{Connection, Error as RusqliteError, Statement, Row, ToSql, NO_PARAMS};
use rusqlite::types::{FromSql, FromSqlResult, ValueRef, ToSqlOutput};

const DEFAULT_FILE: &'static str = "nugmail.db";

pub fn new_connection(file_path: &str) -> Result<Connection, StdError> {
    Ok(Connection::open(file_path)?)
}

pub fn default_connection() -> Result<Connection, StdError> {
    new_connection(DEFAULT_FILE)
}

pub struct Storage<'a> {
    save_address_statement: Statement<'a>,
    delete_address_statement: Statement<'a>,
    get_addresses_statement: Statement<'a>,
    save_message_statement: Statement<'a>,
    get_inbox_statement: Statement<'a>,
}

impl<'a> Storage<'a> {
    pub fn new(connection: &'a Connection) -> Result<Self, StdError> {
        connection.execute("PRAGMA foreign_keys=on", NO_PARAMS).unwrap();

        Ok(
            Storage {
                save_address_statement: connection.prepare("INSERT OR IGNORE INTO email_addresses (address) VALUES (:address)")?,
                delete_address_statement: connection.prepare("DELETE FROM email_addresses WHERE address=:address")?,
                get_addresses_statement: connection.prepare("SELECT * FROM email_addresses")?,
                save_message_statement: connection.prepare("INSERT OR IGNORE INTO emails (remote_id, sender, recipient, subject, body, received_at) VALUES (:remote_id, :sender, :recipient, :subject, :body, :received_at)")?,
                get_inbox_statement: connection.prepare("SELECT * FROM emails WHERE recipient=:address")?,
            }
        )
    }
}

impl Store for Storage<'_> {
    fn save_address(&mut self, address: &Address) -> Result<(), Error> {
        self.save_address_statement.execute_named(&[(":address", address)])?;
        Ok(())
    }

    fn addresses(&mut self) -> Result<Addresses, Error> {
        let mut rows = self.get_addresses_statement.query(NO_PARAMS)?;

        let mut addresses = Addresses::new();
        while let Some(row) = rows.next()? {
            addresses.push(row.get_unwrap::<_, Address>(1))
        }

        Ok(addresses)
    }

    fn delete_address(&mut self, address: &Address) -> Result<(), Error> {
        self.delete_address_statement.execute_named(&[(":address", address)])?;
        Ok(())
    }

    fn save_inbox(&mut self, inbox: &Inbox) -> Result<(), Error> {
        for message in inbox.messages().iter() {
            self.save_message_statement.execute_named(&[
                (":remote_id", message.remote_id()),
                (":sender", message.sender()),
                (":recipient", message.recipient()),
                (":subject", message.subject().unwrap()),
                (":body", message.body().unwrap()),
                (":received_at", message.received_at()),
            ])?;
        }

        Ok(())
    }

    fn inbox(&mut self, address: &Address) -> Result<Inbox, Error> {
        let rows = self.get_inbox_statement.query_map_named::<Message, _>(
            &[(":address", address)],
            |row| Ok(Message::from(row)),
        )?;

        let mut messages = Messages::new();
        for row in rows.into_iter() {
            messages.push(row.unwrap());
        }

        Ok(Inbox::new(address.clone(), messages))
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
