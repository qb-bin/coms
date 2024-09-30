/*

main file - rs/main.rs
Author: qb (kalyan.raparthi@hotmail.com)

*/

use std::net::TcpListener;
use std::io::{Write, Read};

fn main() {
    //buffer with size 512
    let mut buf = [0; 512];

    let l1 = TcpListener::bind("127.0.0.1:2024").unwrap();
    println!("[socket intiated] addr: {}", l1.local_addr().unwrap());
    
    loop {
        let (mut stream, addr) = l1.accept().unwrap();
        println!("{0} connected", addr);

        stream.write_all(&mut buf);

        stream.read(&mut buf);
        println!("{:?}", buf);
    }
}
