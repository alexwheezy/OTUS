use std::io::Read;

use file::open_read;

fn main() {
    let mut file = open_read();
    loop {
        let mut buf = [0; 1024];
        if let Ok(read) = file.read(&mut buf) {
            let str = String::from_utf8_lossy(&buf);
            if read != 0 {
                print!("{}", str);
            }
        }
    }
}
