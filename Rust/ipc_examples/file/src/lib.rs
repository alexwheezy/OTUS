use std::{
    fs,
    io::{Seek, SeekFrom},
    path::Path,
};

pub fn open_write() -> fs::File {
    let path = Path::new("target/tmp/msg.txt");
    let dir = path.parent().unwrap();
    fs::create_dir_all(dir).expect("can't create directory");
    fs::OpenOptions::new()
        .create(true)
        .write(true)
        .open(path)
        .expect("can't open file for write")
}

pub fn open_read() -> fs::File {
    let path = Path::new("target/tmp/msg.txt");
    let mut file = fs::OpenOptions::new()
        .read(true)
        .write(true)
        .open(path)
        .expect("can't open file for read");
    file.seek(SeekFrom::End(0)).expect("can't seek read file");
    file
}
