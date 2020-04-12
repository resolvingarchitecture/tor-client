extern crate log;
extern crate simple_logger;

use log::{trace,info};
use clap::{crate_version, App, Arg, AppSettings};
use tor_client::TORClient;
use ra_common::models::{Envelope, Packet, PacketType, NetworkId};
use ra_common::models::NetworkId::TOR;

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
                    Arg::with_name("directory")
                        .help("directory for saving web resources")
                        .long("dir")
                        .required(true)
                        .takes_value(true),
                    Arg::with_name("max_attempts")
                        .help("maximum number of sends until an ack is received")
                        .long("max_attempts")
                        .takes_value(true),
                ])
        )
        .get_matches();

    match m.subcommand_name() {
        Some("site") => {
            let am = m.subcommand().1.unwrap();
            let url = am.value_of("url").unwrap();
            let dir = am.value_of("dir").unwrap();
            let mut max_attempts :u8 = 1;
            if am.value_of("max_attempts").is_some() {
                max_attempts = am.value_of("max_attempts").unwrap().parse().unwrap();
            }
            site(url, dir, max_attempts);
        },
        None => {
            println!("No subcommand was used")
        },
        _ => println!("Some other subcommand was used"),
    }

}

fn site(url: &str, dir: &str, max_attempts: u8) {
    println!("requesting site...");
    match TORClient::new() {
        Ok(mut client) => {
            for i in 0..max_attempts {
                let env = Envelope::new(0, 0, url.into_bytes());
                let packet = Packet::new(
                    i,
                    PacketType::Data as u8,
                    NetworkId::TOR as u8,
                    client.local_addr.clone(),
                    String::from(url),
                    Some(env));
                println!("Sending msg...");
                client.send(packet);
                println!("Send successful")
            }
        },
        Err(e) => println!("{}", e),
        _ => {}
    }
}