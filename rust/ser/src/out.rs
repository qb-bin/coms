pub fn print_buf(buffer: &[u8; 512]) {
        let mut i = 0;
        while buffer[i] != 0 {
            print!("{} ", buffer[i]);
            i += 1;
        }
}

