use log::{debug,info};
use std::io::{Error, BufReader, Write, BufRead};

use std::net::{TcpStream, ToSocketAddrs};
use std::collections::HashMap;

static DEFAULT_SOCKS: &'static str = "127.0.0.1:9050";
static DEFAULT_CONTROL: &'static str = "127.0.0.1:9051";
static DEFAULT_BROWSER: &'static str = "127.0.0.1:9150";
static DEFAULT_HIDDEN_SERVICE: &'static str = "127.0.0.1:9151";

pub struct TORConnection {
    conn: TcpStream
}

impl TORConnection {
    // pub fn connect<A: ToSocketAddrs>(addr: A) -> Result<TORConnection, Error> {
    //     let mut conn = TORConnection {
    //         conn: TcpStream::connect(addr)?
    //     };
    //
    // }

    // fn handshake(&mut self) -> Result<HashMap<String, String>, Error> {
    //     let hello_msg = format!("HELLO VERSION MIN={} MAX={} \n", self.min_version, self.max_version);
    //     self.send(hello_msg, sam_hello)
    // }
    //
    // fn send<F>(&mut self, msg: String, reply_parser: F) -> Result<HashMap<String, String>, Error>
    //     where
    //         F: Fn(&str) -> IResult<&str, Vec<(&str, &str)>>,
    // {
    //     debug!("-> {}", &msg);
    //     self.conn.write_all(&msg.into_bytes())?;
    //
    //     let mut reader = BufReader::new(&self.conn);
    //     let mut buffer = String::new();
    //     reader.read_line(&mut buffer)?;
    //     debug!("<- {}", &buffer);
    //
    //     let response = reply_parser(&buffer);
    //     let vec = response.unwrap();
    //     let vec_opts = vec.1;
    //     verify_response(&vec_opts).map(|m| {
    //         m.iter()
    //             .map(|(k, v)| (k.to_string(), v.to_string()))
    //             .collect()
    //     })
    // }
}

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
