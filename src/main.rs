mod d7;
use d7 as d;
use std::{fs::File, io::Read};

fn read(file: &str) -> String {
    let mut f = File::open(format!("inputs/{file}.txt")).unwrap();
    let mut buf = String::new();
    f.read_to_string(&mut buf).unwrap();
    buf
}

fn main() {
    assert_eq!(d::p1(&read("test")), 95437);
    println!("{}", d::p1(&read("real")));
    assert_eq!(d::p2(&read("test")), 24933642);
    println!("{}", d::p2(&read("real")));
}
