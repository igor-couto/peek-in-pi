use std::fs::{self};
use std::io;

pub fn open_one_million_file() -> io::Result<String> {
    fs::read_to_string("pi-one-million.txt")
}
