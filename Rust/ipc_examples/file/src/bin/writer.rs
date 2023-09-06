use std::io::{stdin, Write};

use file::open_write;

fn main() {
    let mut file = open_write();
    loop {
        let mut str_buf = String::new();
        stdin().read_line(&mut str_buf).unwrap();
        file.write_all(str_buf.as_bytes()).unwrap();
    }
}
