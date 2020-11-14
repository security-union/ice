use super::{url_parse, STUN_PORT};
use std::net::{SocketAddr, UdpSocket};

pub enum Nat {
    FullCone,
    AddressRestrictedCone,
    PortRestrictedCone,
    Symmetric,
}

#[derive(Debug)]
pub struct Client {
    server: Option<SocketAddr>,
    pub client: UdpSocket,
}

impl Client {
    pub fn new(uri: Option<&str>) -> Result<Self, &'static str> {
        let url: String = match uri {
            Some(u) => u.to_owned(),
            None => format!("stun://127.0.0.1:{}", STUN_PORT),
        };

        let socket_addr = url_parse(&url).expect("local uri format error.");
        let client_socket = UdpSocket::bind(socket_addr).expect("Couldn't bind port");

        match socket_addr.ip().is_loopback() {
            true => Ok(Client {
                server: None,
                client: client_socket,
            }),
            _ => Err("local_uri ip error."),
        }
    }
    pub fn set_server_uri(&mut self, uri: &str) -> bool {
        let stun_server_socket_addr = url_parse(uri).expect("server uri format error.");
        self.server = Some(stun_server_socket_addr);
        true
    }
    pub fn send(&self, msg: &[u8]) -> Result<usize, &'static str> {
        assert_eq!(self.server.is_some(), true);
        let target = self.server.unwrap();
        match self.client.send_to(msg, target) {
            Ok(size) => Ok(size),
            Err(_) => Err("send error."),
        }
    }
    pub fn nat(&self) {
        assert_eq!(self.server.is_some(), true);
    }
}
