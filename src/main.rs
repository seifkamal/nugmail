mod email;
mod webhook_site;

use clap::{App, AppSettings, SubCommand};
use crate::email::Service;

type StdError = Box<dyn std::error::Error>;

fn main() {
    let matches = App::new("nug")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(SubCommand::with_name("new"))
        .get_matches();

    match matches.subcommand_name().unwrap() {
        "new" => {
            let client = webhook_site::Client::new();
            let email = client.generate().unwrap();

            println!("{}", email);
        }
        _ => unreachable!(),
    }
}
