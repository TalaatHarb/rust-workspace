use crate::http::*;
use std::{
    convert::TryFrom,
    io::Read,
    net::{
        TcpListener,
        TcpStream
    }
};

#[derive(Debug)]
pub struct HttpServer {
    ip: String,
    port: u16,
}

impl HttpServer {
    const MAX_PORT: u16 = 49152;

    pub fn new(listen_ip: &str, listen_port: u16) -> Self {
        // TODO add 'validator' package to handle validation of port and IP
        assert!(
            HttpServer::MAX_PORT > listen_port,
            "Server ports should be less than {}",
            HttpServer::MAX_PORT
        );
        return HttpServer {
            ip: String::from(listen_ip),
            port: listen_port,
        };
    }

    pub fn run(self) {
        let sock_address = get_socket_address(&self.ip, self.port);

        let tcp_listner: TcpListener = TcpListener::bind(&sock_address).unwrap();
        println!("Listening on ip = {} and port = {}", self.ip, self.port);

        loop {
            let connection: Result<(TcpStream, std::net::SocketAddr), std::io::Error> = tcp_listner.accept();
            match connection {
                Ok((mut tcp_stream, peer_address)) => {
                    println!("Connection established to {}", &peer_address.to_string());

                    let mut buffer: [u8; 1024] = [0; 1024];
                    match  tcp_stream.read(&mut buffer){
                        Ok(n) =>{
                            println!("-----------------------------------------------------------------------------------------");
                            println!("Read {} bytes from {}", n, &peer_address.to_string());
                            match HttpRequest::try_from(&buffer[..]){
                                Ok(request) => {
                                    dbg!(&request);
                                },
                                Err(err) => {
                                    eprintln!("Unable to parse buffer into an HttpRequest due to {err}");
                                }
                            }
                            println!("-----------------------------------------------------------------------------------------");
                        },
                        Err(err) =>{
                            eprintln!("Failed to read bytes from {}, due to {}", &peer_address.to_string(), err);
                        }
                    }
                },
                Err(err) => {
                    eprintln!("Failed connection attempt due to {err}");
                }
            }
        }
    }

    
}

fn get_socket_address(ip: &str, port: u16) -> String {
    let mut sock_address: String = String::new();
    sock_address.push_str(ip);
    sock_address.push(':');
    sock_address.push_str(&port.to_string());

    return sock_address;
}