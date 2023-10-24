use std::net::{TcpStream, ToSocketAddrs};
use std::io::{self, Write};
use std::time::Duration;

pub fn check_tcp_connection(host: &str, port: &str) {
    let addr = format!("{}:{}", host, port);
    let addrs = addr.to_socket_addrs().unwrap().next().unwrap();
    let timeout = Duration::new(5, 0);
    match TcpStream::connect_timeout(&addrs, timeout) {
        Ok(stream) => { // bind the result to `stream`
            writeln!(io::stdout(), "Successfull TCP connection to {}:{}", host, port).unwrap();
            writeln!(io::stdout(), "Stream local address: {:?}", stream.local_addr().unwrap()).unwrap();
            writeln!(io::stdout(), "Stream peer address: {:?}", stream.peer_addr().unwrap()).unwrap();
            writeln!(io::stdout(), "Stream is readable: {:?}", stream.set_read_timeout(Some(std::time::Duration::new(1, 0)))).unwrap();
            writeln!(io::stdout(), "Stream is writable: {:?}", stream.set_write_timeout(Some(std::time::Duration::new(1, 0)))).unwrap();
        }
        Err(e) => {
            writeln!(io::stderr(), "Failed TCP connection to {}:{}: {}", host, port, e).unwrap();
            writeln!(io::stderr(), "Timeout was set to {} seconds", timeout.as_secs()).unwrap();
            writeln!(io::stderr(), "Error: {}", e).unwrap();
        }
    }
}