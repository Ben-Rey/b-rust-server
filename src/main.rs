use server::Server;
use http::Method;

mod http;
mod server;

fn main() {
    let get = Method::GET;
    let put = Method::PUT;
    let post = Method::POST;
    let delete = Method::DELETE;

    let server = Server::new("127.0.0.1:8080".to_string());
    server.run();

}