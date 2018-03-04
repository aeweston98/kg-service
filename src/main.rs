extern crate mio;
extern crate http_muncher;
extern crate sha1;
extern crate rustc_serialize;

use std::collections::HashMap;
use std::net::SocketAddr;

use mio::*;
use mio::tcp::*;

mod server;
mod kg_client;

fn main() {
	/*
    let address = "0.0.0.0:10000".parse::<SocketAddr>().unwrap();
    let server_socket = TcpListener::bind(&address).unwrap();

    let mut event_loop = EventLoop::new().unwrap();

    let mut _server = server::WebSocketServer {
        token_counter: 1,
        clients: HashMap::new(),
        socket: server_socket
    };

    event_loop.register(&_server.socket,
                        server::SERVER_TOKEN,
                        EventSet::readable(),
                        PollOpt::edge()).unwrap();
    event_loop.run(&mut _server).unwrap();
    */

    let mut client = kg_client::GoogleGraphClient{ base_url: String::from("https://kgsearch.googleapis.com/v1/entities:search/")}
    let request = lg_client::Request::new(String::from("Purple Haze", 10));

    
}
