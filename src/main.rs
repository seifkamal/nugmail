mod tui;

use clap::{App, AppSettings, Arg, SubCommand};

use nugmail::{
    generator::{webhook_site, Service},
    storage::{sqlite, Store},
};

fn main() {
    let matches = App::new("Nugmail")
        .about("A client for generating disposable email addresses")
        .version("0.1.0")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommands(vec![
            SubCommand::with_name("list")
                .about("Display all generated addresses")
                .version("0.1.0"),
            SubCommand::with_name("new")
                .about("Generate a new address")
                .version("0.1.0"),
            SubCommand::with_name("inbox")
                .about("View an address inbox")
                .version("0.1.0")
                .arg(
                    Arg::with_name("address")
                        .help("The email address to view the inbox for")
                        .short("a")
                        .required(true)
                        .takes_value(true)
                        .number_of_values(1),
                ),
            SubCommand::with_name("delete")
                .about("Delete an address and its inbox")
                .version("0.1.0")
                .arg(
                    Arg::with_name("address")
                        .help("The email address to view the inbox for")
                        .short("a")
                        .required(true)
                        .takes_value(true)
                        .number_of_values(1),
                ),
        ])
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
            let client = webhook_site::Client::default();
            let address = client.generate().unwrap();

            println!("{}", address);
            storage.save_address(&address).unwrap();
        }
        ("delete", Some(cmd)) => match storage.address(cmd.value_of("address").unwrap()) {
            Ok(address) => {
                let client = webhook_site::Client::default();

                client.delete(&address).unwrap();
                storage.delete_address(&address).unwrap();
                println!("Address successfully deleted");
            }
            Err(_) => println!("Address does not exist"),
        },
        ("inbox", Some(cmd)) => match storage.address(cmd.value_of("address").unwrap()) {
            Ok(address) => tui::render_inbox(address, storage, webhook_site::Client::default()),
            Err(_) => println!("Address does not exist"),
        },
        _ => unreachable!(),
    }
}
