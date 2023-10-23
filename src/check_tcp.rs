use std::net::TcpStream;
use std::io::{self, Write};

pub fn check_tcp_connection(host: &str, port: &str) {
    match TcpStream::connect(format!("{}:{}", host, port)) {
        Ok(stream) => { // bind the result to `stream`
            writeln!(io::stdout(), "Successfully connected to {}:{}", host, port).unwrap();
            writeln!(io::stdout(), "Stream local address: {:?}", stream.local_addr().unwrap()).unwrap();
            writeln!(io::stdout(), "Stream peer address: {:?}", stream.peer_addr().unwrap()).unwrap();
            writeln!(io::stdout(), "Stream is readable: {:?}", stream.set_read_timeout(Some(std::time::Duration::new(1, 0)))).unwrap();
            writeln!(io::stdout(), "Stream is writable: {:?}", stream.set_write_timeout(Some(std::time::Duration::new(1, 0)))).unwrap();
        }
        Err(e) => {
            writeln!(io::stderr(), "Failed to connect to {}:{}: {}", host, port, e).unwrap();
            writeln!(io::stderr(), "Error: {}", e).unwrap();
        }
    }
}