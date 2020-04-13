use log::{info};
use std::io::Error;

pub struct TORClient {
    pub local_addr: String
}

impl TORClient {
    pub fn new() -> Result<TORClient, Error> {
        Ok(TORClient {
            local_addr: String::new()
        })
    }
    pub fn get(&mut self, url: &String) -> Result<Vec<u8>, Error> {
        info!("Get resource {}...", &url);
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
