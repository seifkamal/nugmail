use crate::StdError;
use crate::email::{Address, Inbox, Messages, Message};
use crate::storage::{Store, Error};
use rusqlite::{Connection, Error as RusqliteError, Statement, Row};
use rusqlite::types::{FromSql, FromSqlResult, ValueRef};

const DEFAULT_FILE: &'static str = "nugmail.db";

pub fn new_connection(file_path: &str) -> Result<Connection, StdError> {
    Ok(Connection::open(file_path)?)
}

pub fn default_connection() -> Result<Connection, StdError> {
    new_connection(DEFAULT_FILE)
}

pub struct EmailStorage<'a> {
    save_address_statement: Statement<'a>,
    get_address_inbox_statement: Statement<'a>,
}

impl<'a> EmailStorage<'a> {
    pub fn new<'b>(connection: &'a Connection) -> Result<Self, StdError> {
        Ok(
            EmailStorage {
                save_address_statement: connection.prepare("INSERT OR IGNORE INTO email_addresses (address) VALUES (:address)")?,
                get_address_inbox_statement: connection.prepare("SELECT * FROM emails WHERE recipient=:address")?,
            }
        )
    }
}

impl Store for EmailStorage<'_> {
    fn save(&mut self, address: Address) -> Result<(), Error> {
        self.save_address_statement.execute_named(&[(":address", address.as_str())])?;
        Ok(())
    }

    fn inbox(&mut self, address: Address) -> Result<Inbox, Error> {
        let rows = self.get_address_inbox_statement.query_map_named::<Message, _>(
            &[(":address", address.as_str())],
            |row| Ok(Message::from(row))
        )?;

        let mut messages = Messages::new();
        for row in rows.into_iter() {
            messages.push(row.unwrap());
        }

        Ok(Inbox::new(address, messages))
    }
}

impl FromSql for Address {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        Ok(Address::new(value.as_str()?))
    }
}

impl From<&Row<'_>> for Message {
    fn from(row: &Row<'_>) -> Self {
        Message::new(
            row.get_unwrap::<_, Address>(1),
            row.get_unwrap::<_, Address>(2),
            Some(row.get_unwrap(3)),
            Some(row.get_unwrap(4)),
            row.get_unwrap(5),
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
