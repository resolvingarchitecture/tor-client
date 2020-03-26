use ra_common::models::{Packet};

use log::{info};

pub struct TORClient {

}

impl TORClient {
    pub fn new() -> TORClient {
        TORClient {

        }
    }
    pub fn init(&mut self) {
        info!("{}","Initializing TOR Client...");

    }
    pub fn handle(&mut self, packet: &mut Packet) {
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
