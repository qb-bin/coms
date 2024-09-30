pub mod out;
use out::print_buf;

use std::net::TcpStream;
use std::io::{stdin, Read, Write};


fn main() {
    let addr = "127.0.0.1:2024";
    let mut buf = [0; 512];   
    let mut str_io = String::new();

    // let mut stream = TcpStream::connect(addr).expect("error while connecting\n");
    // println!("connected to {addr}");

    print!("> ");
    loop {
        let _ = stdin().read_line(&mut str_io);
        println!("{}", str_io);

        // print_buf(&buf);
        // let _ = stream.write_all(&mut buf);

        // let _ = stream.read(&mut buf);
        // print_buf(&buf);    
    }
}

