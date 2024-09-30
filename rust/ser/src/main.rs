pub mod out;
use out::print_buf;

use std::net::TcpListener;
use std::io::{Read, Write, stdin};

fn main() {
    let mut buf = [0; 512];
    let l1 = TcpListener::bind("127.0.0.1:2024").unwrap();
    
    let (mut stream, addr) = l1.accept().unwrap();
    println!("connected : {}", addr);
}
