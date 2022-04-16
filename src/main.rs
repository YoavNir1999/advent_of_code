mod parse_text;
use parse_text::*;
use std::io::prelude::*;


fn main() {
    let file = open("/Users/yoavnir/Documents/vs code/rust/old/solving_challenges/advent_of_code/text.txt");

    for line in file_to_iter(file).lines() {
        let line = match line {
            Ok(line) => line,
            Err(error) => panic!()
        };
        
    }
}

