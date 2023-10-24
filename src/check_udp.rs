use std::net::UdpSocket;
use std::io::{self, Write};

pub fn check_udp_connection(host: &str, port: &str) {
    match UdpSocket::bind(format!("{}:{}", host, port)) {
        Ok(socket) => {
            writeln!(io::stdout(), "Successfull UDP connection to {}:{}", host, port).unwrap();
            writeln!(io::stdout(), "Socket local address: {:?}", socket.local_addr().unwrap()).unwrap();
        }
        Err(e) => {
            writeln!(io::stderr(), "Failed UDP connection to {}:{}: {}", host, port, e).unwrap();
            writeln!(io::stderr(), "Error: {}", e).unwrap();
        }
    }
}