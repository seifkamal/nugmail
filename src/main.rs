mod tui;

use clap::{App, AppSettings, Arg, SubCommand};

use nugmail::{
    email::Address,
    generator::{Service, webhook_site},
    storage::{Store, sqlite},
};

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
        .subcommand(
            SubCommand::with_name("delete")
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

    let mut storage: sqlite::Storage = Default::default();

    match matches.subcommand() {
        ("list", _) => {
            let addresses = storage.addresses().unwrap();
            for address in addresses.iter() {
                println!("{}", address);
            }
        }
        ("new", _) => {
            let client = webhook_site::Client::new();
            let address = client.generate().unwrap();

            println!("{}", address);
            storage.save_address(&address).unwrap();
        }
        ("delete", Some(cmd)) => {
            let address = Address::from(cmd.value_of("address").unwrap());
            let client = webhook_site::Client::new();

            client.delete(&address).unwrap();
            storage.delete_address(&address).unwrap();
            println!("Address successfully deleted");
        }
        ("inbox", Some(cmd)) => {
            let address = Address::from(cmd.value_of("address").unwrap());
            let client = webhook_site::Client::new();

            tui::render_inbox(address, storage, client);
        }
        _ => unreachable!(),
    }
}
