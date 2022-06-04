use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::path::Path;

pub fn solution(){
    let path = Path::new("files/1.txt").canonicalize().unwrap();

    let f = File::open(path).expect("file not found");

    let reader = BufReader::new(f);

    let mut lines = reader.lines();
    let mut prev_line = lines.next().unwrap().unwrap();

    let mut count = 0_usize;
    for (_, line) in lines.enumerate() {
        let line = line.unwrap();
        let value = line.parse::<i32>().unwrap();
        let prev_value = prev_line.parse::<i32>().unwrap();
        if prev_value < value {
            count += 1;
        }
        prev_line = line;
    }
    println!("{}", count);
}