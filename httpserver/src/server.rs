use std::{io::Read, net::TcpListener};

use http::httprequest::HttpRequest;

use crate::router::Router;

pub struct Server<'a> {
    socket_addr: &'a str,
}

impl<'a> Server<'a> {
    pub fn new(socket_addr: &'a str) -> Server {
        Server { socket_addr }
    }

    pub fn run(&self) {
        let listener = TcpListener::bind(self.socket_addr).unwrap();
        println!("Running on {}", self.socket_addr);
        for stream in listener.incoming() {
            let mut stream = stream.unwrap();
            println!("Connection established");

            let mut read_buffer = [0; 1024];
            stream.read(&mut read_buffer).unwrap();
            let req: HttpRequest = String::from_utf8(read_buffer.to_vec()).unwrap().into();
            Router::route(req, &mut stream);
        }
    }
}
