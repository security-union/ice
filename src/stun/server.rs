use std::io::{Read, Write};
use std::net::{SocketAddr, TcpListener, UdpSocket};

use super::packet;
use super::url_parse;

pub fn handler(
    msg: &[u8],
    response: &mut [u8],
    peer_socket_addr: &SocketAddr,
    local_socket_addr: &SocketAddr,
) -> Result<usize, &'static str> {
    println!(
        "[Handler] Local Addr: {:?} <-- Peer Addr: {:?}",
        local_socket_addr, peer_socket_addr
    );

    match packet::Header::from_bytes(&msg[..20]) {
        Ok(mut head) => {
            println!("[DEBUG] STUN Request Head: {:?}", head);

            let attr = packet::Attribute::MappedAddress(peer_socket_addr.clone());

            let attr_bytes: Vec<u8> = attr.into_bytes();
            let attr_length = attr_bytes.len() as u16;

            head.set_class(packet::header::Class::SuccessResponse);
            head.set_length(attr_length);

            let mut stun_packet: Vec<u8> = vec![];

            stun_packet.extend(head.into_bytes());
            stun_packet.extend(attr_bytes);
            for idx in 0..stun_packet.len() {
                response[idx] = stun_packet[idx];
            }
            println!("[DEBUG] STUN Response: {:?}", stun_packet);
            Ok(stun_packet.len())
        }
        Err(e) => Err(e),
    }
}

pub fn tcp_server(host: &str) {
    let socket_addr = url_parse(host).expect("local uri format error.");
    let listener = TcpListener::bind(socket_addr).unwrap();
    println!("[TCP Server] server running at : {:?}", listener);

    let mut buf = [0; 2048];
    let mut response = [0; 2048];

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                // thread::spawn(move || tcp_handler(&mut stream));
                match stream.read(&mut buf[..]) {
                    Ok(size) => {
                        let msg = &buf[..size];
                        match handler(
                            &msg,
                            &mut response,
                            &stream.peer_addr().unwrap(),
                            &socket_addr,
                        ) {
                            Ok(size) => {
                                if size > 0 {
                                    match stream.write(&response[..size]) {
                                        Ok(_size) => {},
                                        Err(e) => println!("[Error] {:?}", e)
                                    }
                                }
                            }
                            Err(e) => println!("[Error] {:?}", e),
                        }
                    }
                    Err(e) => println!("[Error] {:?}", e),
                };
            }
            Err(e) => println!("[Error] {:?}", e),
        };
    }
}

pub fn udp_server(host: &str) {
    let socket_addr = url_parse(host).expect("local uri format error.");
    let socket = UdpSocket::bind(socket_addr).unwrap();
    println!("[UDP Server] server running on {} ...", socket_addr);
    let mut buf = [0; 2048];
    let mut response = [0; 2048];
    loop {
        match socket.recv_from(&mut buf) {
            Ok((size, peer_socket_addr)) => {
                println!("[INFO] Connection: {:?}", peer_socket_addr);
                let msg = &buf[..size];
                // thread::spawn(move || handler(&msg, &mut response));
                match handler(&msg, &mut response, &peer_socket_addr, &socket_addr) {
                    Ok(size) => {
                        if size > 0 {
                            match socket.send_to(&response[..size], &peer_socket_addr) {
                                Ok(_size) => {},
                                Err(e) => println!("[Error] {:?}", e),
                            }
                        }
                    }
                    Err(e) => println!("[Error] {:?}", e),
                }
            }
            Err(e) => println!("[Error] {:?}", e),
        };
    }
}

pub fn run(host: &str, protocol: &str) {
    match protocol.to_lowercase().as_str() {
        "tcp" => tcp_server(host),
        "udp" => udp_server(host),
        _ => panic!("[Error] protocol error {:?}", protocol),
    }
}
