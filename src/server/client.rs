extern crate mio;
extern crate http_muncher;
extern crate sha1;
extern crate rustc_serialize;

use std::collections::HashMap;
use std::cell::RefCell;
use std::rc::Rc;
use std::fmt;
use server::parser::HttpParser;
use mio::tcp::TcpStream;

fn gen_key(key: &String) -> String {
    use rustc_serialize::base64::STANDARD;
    use rustc_serialize::base64::ToBase64;

    let mut m = sha1::Sha1::new();
    let mut buf = [0u8; 20];

    m.update(key.as_bytes());
    m.update("258EAFA5-E914-47DA-95CA-C5AB0DC85B11".as_bytes());

    m.output(&mut buf);

    return buf.to_base64(STANDARD);
}


#[derive(PartialEq)]
pub enum ClientState {
    AwaitingHandshake,
    HandshakeResponse,
    Connected
}

pub struct WebSocketClient {
    pub socket: TcpStream,
    pub headers: Rc<RefCell<HashMap<String, String>>>,
    pub http_parser: http_muncher::Parser<HttpParser>,
    pub interest: ::EventSet,
    pub state: ClientState
}

impl WebSocketClient {
    pub fn new(socket: TcpStream) -> WebSocketClient {
        let headers = Rc::new(RefCell::new(HashMap::new()));

        WebSocketClient {
            socket: socket,
            headers: headers.clone(),
            http_parser: http_muncher::Parser::request(HttpParser {
                current_key: None,
                headers: headers.clone()
            }),
            interest: ::EventSet::readable(),
            state: ClientState::AwaitingHandshake
        }
    }

    pub fn write(&mut self) {
        use mio::TryWrite;

        let headers = self.headers.borrow();
        let response_key = gen_key(&headers.get("Sec-WebSocket-Key").unwrap());
        let response = fmt::format(format_args!("HTTP/1.1 101 Switching Protocols\r\n\
                                                 Connection: Upgrade\r\n\
                                                 Sec-WebSocket-Accept: {}\r\n\
                                                 Upgrade: websocket\r\n\r\n", response_key));
        self.socket.try_write(response.as_bytes()).unwrap();

        // Change the state
        self.state = ClientState::Connected;

        self.interest.remove(::EventSet::writable());
        self.interest.insert(::EventSet::readable());
    }

    pub fn read(&mut self) {
        use mio::TryRead;
        loop {
            let mut buf = [0; 2048];
            match self.socket.try_read(&mut buf) {
                Err(e) => {
                    println!("Error while reading socket: {:?}", e);
                    return
                },
                Ok(None) =>
                    // Socket buffer has got no more bytes.
                    break,
                Ok(Some(len)) => {
                    self.http_parser.parse(&buf);
                    if self.http_parser.is_upgrade() {
                        // Change the current state
                        self.state = ClientState::HandshakeResponse;

                        // Change current interest to `Writable`
                        self.interest.remove(::EventSet::readable());
                        self.interest.insert(::EventSet::writable());
                        break;
                    }
                }
            }
        }
    }
}