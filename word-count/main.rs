#![allow(dead_code)]

use std::error::Error;
use std::fs::File;
use std::path::Path;
use std::io::Read;

fn word_count(s:String) -> i64 {
    let mut cnt:i64 = 0i64;
    for _i in s.split_whitespace() {
        cnt += 1;
    }
    cnt
}

fn main() -> std::io::Result<()> {
    let path = Path::new("words.txt");
    let mut file = match File::open(&path) {
        Ok(file) => file,
        Err(err) => panic!("{}", err.description())
    };
    let mut s:String = String::new();
    match file.read_to_string(&mut s) {
        Ok(_) => println!("{}", word_count(s)),
        Err(err) => panic!("{}", err.description())
    }
    Ok(())
    
}
