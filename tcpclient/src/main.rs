use std::net::TcpStream;

fn main() {
    let _stream = TcpStream::connect("127.0.0.1:3000");
    println!("Hello, world!");
}
