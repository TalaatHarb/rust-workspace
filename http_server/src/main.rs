mod server;
mod http;
use server::HttpServer;

fn main() {
    let server: HttpServer = HttpServer::new("0.0.0.0", 8080);
    server.run();
}