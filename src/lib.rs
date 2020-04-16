use log::{debug,info};
use std::io::{Error, BufReader, Write, BufRead, Read, ErrorKind};

use std::net::{TcpStream, ToSocketAddrs};
use std::collections::HashMap;
use std::time::SystemTime;
use byteorder::{LittleEndian, BigEndian, WriteBytesExt};

static DEFAULT_SOCKS: &'static str = "127.0.0.1:9050";
static HELLO: &'static [u8] = &[0x05,0x01,0x00];

static DEFAULT_CONTROL: &'static str = "127.0.0.1:9051";

static DEFAULT_BROWSER: &'static str = "127.0.0.1:9150";

static DEFAULT_HIDDEN_SERVICE: &'static str = "127.0.0.1:9151";

pub struct TORConnection {
    tcp_stream: TcpStream
}

impl TORConnection {
    pub fn connect<A: ToSocketAddrs>(addr: A) -> Result<TORConnection, Error> {
        let tcp_stream = TcpStream::connect(addr)?;
        let mut conn = TORConnection {
            tcp_stream
        };
        match conn.handshake() {
            Ok(m) => return Ok(conn),
            Err(e) => return Err(e)
        }
    }

    fn handshake(&mut self) -> Result<Vec<u8>, Error> {
        debug!("HELLO...");
        let stopwatch = SystemTime::now();
        match self.send(HELLO) {
            Ok(res) => {
                let version: u8 = res.get(0).unwrap().to_ascii_lowercase();
                let cauth: u8 = res.get(1).unwrap().to_ascii_lowercase();
                if res.len() != 2 || version != 5 || cauth != 0 {
                    Err(Error::new(ErrorKind::InvalidData, format!("Error from initial handshake")))
                } else {
                    let mark = stopwatch.elapsed().unwrap();
                    debug!("HELLO THERE ({} seconds).", mark.as_secs());
                    Ok(res)
                }
            },
            Err(e) => Err(e)
        }
    }

    fn send(&mut self, data: &[u8]) -> Result<Vec<u8>, Error> {
        self.tcp_stream.write_all(data)?;
        let mut reader = BufReader::new(&self.tcp_stream);
        let mut buffer: Vec<u8> = Vec::new();
        reader.read_to_end(&mut buffer)?;
        return Ok(buffer);
    }
}

pub struct Session {
    tor: TORConnection,
    // local_addr: String
}

impl Session {
    pub fn create<A: ToSocketAddrs>(tcp_addr: A) -> Result<Session, Error> {
        let mut tor = TORConnection::connect(tcp_addr)?;
        // let local_addr = String::new();
        Ok(Session {tor})
    }

    pub fn get(&mut self, host_name: &str, url: &str) -> Result<Vec<u8>, Error> {
        debug!("Connection request...");
        let mut port = vec![];
        port.write_u16::<LittleEndian>(80).unwrap();
        debug!("port length: {}", port.len());
        let mut req: Vec<u8> = Vec::new();
        req.push(0x05); // SOCKS version
        req.push(0x01); // TCP/IP Stream
        req.push(0x00); // Reserved - must be zero
        req.push(0x03); // Use Domain name for address
        let host_len = host_name.len();
        debug!("hostname length: {}", host_len);
        req.push(host_len as u8); // Length of host name
        req.write_all(host_name.as_bytes()); // Host name
        req.append(port.as_mut()); // port little endian
        debug!("request length: {}", req.len());
        let stopwatch = SystemTime::now();
        match self.tor.send(&req) {
            Ok(res) => {
                let mark = stopwatch.elapsed().unwrap();
                debug!("Response in {} seconds.", mark.as_secs());
                if res.len() == 0 {
                    return Err(Error::new(ErrorKind::InvalidData, "Response from connection request is empty."));
                }
                let version: u8 = res.get(0).unwrap().to_ascii_lowercase();
                let status: u8 = res.get(1).unwrap().to_ascii_lowercase();
                if version != 5 {
                    return Err(Error::new(ErrorKind::InvalidInput,format!("SOCKS version 5 not supported. Version {} offered.", version)))
                }
                if status != 0 {
                    return Err(Error::new(ErrorKind::InvalidData, format!("Error from initial handshake")))
                }
            },
            Err(e) => return Err(e)
        }
        self.tor.send(url.as_bytes())
    }
}

pub struct TORClient {
    pub session: Session
}

impl TORClient {
    pub fn new() -> Result<TORClient, Error> {
        Ok(TORClient {
            session: Session::create(DEFAULT_SOCKS)?
        })
    }
    pub fn get(&mut self, url: &str) -> Result<Vec<u8>, Error> {
        info!("Get resource {}...", &url);
        self.session.get("1m5.io", "https://1m5.io")
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
