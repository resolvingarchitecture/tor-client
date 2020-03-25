extern crate log;
extern crate simple_logger;

use log::{trace,info};
use tor_client::TORClient;

fn main() {
    simple_logger::init().unwrap();
    trace!("Starting TOR Client Daemon...");
    let mut tor_client = TORClient::new();
    tor_client.init();
    trace!("TOR Client Daemon Stopped.");
}