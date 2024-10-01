pub mod out;
use out::print_buf;

use std::net::TcpListener;
use std::io::{stdin, BufWriter, Read, Write};

fn main() {
    println!("\nserver\n");

    let mut buf:[u8; 512] = [2; 512];

    let l1 = TcpListener::bind("127.0.0.1:2024").unwrap(); 
    let (mut stream, addr) = l1.accept().unwrap();
    println!("connected : {}", addr);
    
    stream.write(&mut buf).unwrap();

    stream.read(&mut buf).unwrap();
    println!("{:?}", buf);
}
