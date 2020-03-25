use ra_common::models::{Network, Packet};

use log::{trace,info};

pub struct TORClient {

}

impl TORClient {
    pub fn new() -> Box<TORClient> {
        Box::new(TORClient {

        })
    }
    pub fn init(&mut self) {
        info!("{}","Initializing TOR Client...");

    }
}

impl Network for TORClient {
    fn handle(&mut self, packet: &mut Packet) {
        info!("Handling incoming packet id={}",packet.id);

    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
