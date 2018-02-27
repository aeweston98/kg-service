extern crate mio;
extern crate http_muncher;
extern crate sha1;
extern crate rustc_serialize;

use std::collections::HashMap;

use mio::*;
use mio::tcp::*;

pub mod client;
pub mod parser;

pub struct WebSocketServer {
    pub socket: TcpListener,
    pub clients: HashMap<Token, client::WebSocketClient>,
    pub token_counter: usize
}

pub const SERVER_TOKEN: Token = Token(0);

impl Handler for WebSocketServer {
    type Timeout = usize;
    type Message = ();

    fn ready(&mut self, event_loop: &mut EventLoop<WebSocketServer>, token: Token, events: EventSet) {
        if events.is_readable() {
            match token {
                SERVER_TOKEN => {
                    let client_socket = match self.socket.accept() {
                        Ok(Some((sock, addr))) => sock,
                        Ok(None) => unreachable!(),
                        Err(e) => {
                            println!("Accept error: {}", e);
                            return;
                        }
                    };

                    let new_token = Token(self.token_counter);
                    self.clients.insert(new_token, client::WebSocketClient::new(client_socket));
                    self.token_counter += 1;

                    event_loop.register(&self.clients[&new_token].socket, new_token, EventSet::readable(),
                                        PollOpt::edge() | PollOpt::oneshot()).unwrap();
                },
	        token => {
                    let mut client = self.clients.get_mut(&token).unwrap();
                    client.read();
                    event_loop.reregister(&client.socket, token, client.interest,
                                          PollOpt::edge() | PollOpt::oneshot()).unwrap();
                }
            }
        }

        if events.is_writable() {
            let mut client = self.clients.get_mut(&token).unwrap();
            client.write();
            event_loop.reregister(&client.socket, token, client.interest,
                                  PollOpt::edge() | PollOpt::oneshot()).unwrap();
        }
    }
}