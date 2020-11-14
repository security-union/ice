extern crate rand;
extern crate url;

pub mod stun;

fn main() {
    let stun_server = "127.0.0.1:3478";

    let data = [
        0, 1, 0, 0, 33, 18, 164, 66, 70, 149, 250, 122, 253, 177, 191, 174, 164, 118, 181,
    ];

    let mut client = stun::client::Client::new(None).unwrap();
    client.set_server_uri(stun_server);

    let res = client.send(&data);

    let mut buf = [0; 19];
    match client.client.recv(&mut buf) {
        Ok(received) => println!("received {} bytes {:?}", received, &buf[..received]),
        Err(e) => println!("recv function failed: {:?}", e),
    }

    println!("Result: {:?}", res);
}
