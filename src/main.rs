mod email;
mod storage;
mod generator;
mod tui;

use crate::email::Address;
use crate::storage::{Store, sqlite};
use crate::generator::Service;
use clap::{App, AppSettings, Arg, SubCommand};

type StdError = Box<dyn std::error::Error>;

fn main() {
    let matches = App::new("nug")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(SubCommand::with_name("list"))
        .subcommand(SubCommand::with_name("new"))
        .subcommand(
            SubCommand::with_name("inbox")
                .arg(
                    Arg::with_name("address")
                        .help("The email address to view the inbox for")
                        .short("a")
                        .required(true)
                        .takes_value(true)
                        .number_of_values(1)
                )
        )
        .get_matches();

    match matches.subcommand() {
        ("list", _) => {
            let storage_connection = sqlite::default_connection().unwrap();
            let mut storage = sqlite::EmailStorage::new(&storage_connection).unwrap();
            let addresses = storage.addresses().unwrap();

            for address in addresses.iter() {
                println!("{}", address)
            }
        }
        ("new", _) => {
            let client = generator::webhook_site::Client::new();
            let address = client.generate().unwrap();
            println!("{}", address);

            let storage_connection = sqlite::default_connection().unwrap();
            let mut storage = sqlite::EmailStorage::new(&storage_connection).unwrap();
            storage.save_address(address).unwrap();
        }
        ("inbox", Some(cmd)) => {
            let address = Address::from(cmd.value_of("address").unwrap());
            let storage_connection = sqlite::default_connection().unwrap();
            let storage = sqlite::EmailStorage::new(&storage_connection).unwrap();
            let client = generator::webhook_site::Client::new();

            tui::render_inbox(address, storage, client)
        }
        _ => unreachable!(),
    }
}
