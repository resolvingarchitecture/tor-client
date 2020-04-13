extern crate log;
extern crate simple_logger;

use clap::{crate_version, App, Arg, AppSettings};
use tor_client::TORClient;
use std::io::{Error, ErrorKind};

fn main() {
    simple_logger::init().unwrap();
    let m = App::new("tor")
        .about("A TOR client for the local TOR router instance. Not compliant with any version yet.")
        .version(crate_version!())
        .author("Brian Taylor <brian@resolvingarchitecture.io>")
        .setting(AppSettings::ArgRequiredElseHelp)
        .setting(AppSettings::ColoredHelp)
        .setting(AppSettings::ColorAlways)
        .subcommand(
            App::new("site")
                .about("a request for a web resource")
                .args(&[
                    Arg::with_name("url")
                        .help("address of web resource")
                        .long("url")
                        .required(true)
                        .takes_value(true),
                ])
        )
        .get_matches();

    match m.subcommand_name() {
        Some("get") => {
            let am = m.subcommand().1.unwrap();
            let url = am.value_of("url").unwrap();
            get(String::from(url));
        },
        None => {
            println!("No subcommand was used")
        },
        _ => println!("Some other subcommand was used"),
    }

}

fn get(url: String) -> Result<Vec<u8>, Error> {
    println!("get...");
    let mut client = TORClient::new()?;
    client.get(&url)
}