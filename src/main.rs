mod parse_text;
use parse_text::*;
use std::io::prelude::*;
use md5;


fn main() {
    let file = open("/Users/yoavnir/Documents/vs code/rust/old/solving_challenges/advent_of_code/text.txt");

    let lines = file_to_iter(file);

    let x = 0;
    let y = 1;
    let and = x&y;
    println!("{and}");

    for line in lines.lines() {
        let line = match line {
            Ok(line) => line,
            Err(err) => panic!("{}",err)
        };


    }
}