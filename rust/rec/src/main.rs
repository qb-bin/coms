use std::net::TcpStream;
use std::io::{Write, Read};

fn main() {

    let addr = "127.0.0.1:2024";
    let mut buf = [0; 512];   

    buf[0] = 65;
    buf[1] = 66;
    buf[2] = 67;

    let mut stream = TcpStream::connect(addr).expect("error while connecting\n");
    println!("connected to {addr}");

    //loop {
        stream.write(&mut buf);
    //}

}
