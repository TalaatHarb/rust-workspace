fn main() {
    let server: HttpServer = HttpServer::new("127.0.0.1", 8080);
    server.run();
}

struct HttpServer{
    ip: String,
    port: u16
}

impl HttpServer{
    const MAX_PORT: u16 = 49152;

    fn new(listen_ip: &str, listen_port: u16) -> Self{
        // TODO add 'validator' package to handle validation of port and IP
        assert!( HttpServer::MAX_PORT > listen_port, "Server ports should be less than {}", HttpServer::MAX_PORT);
        return HttpServer { ip: String::from(listen_ip), port: listen_port };
    }

    fn run(self){
        println!("Listening on ip = {} and port = {}", self.ip, self.port);
    }
}