mod parse_text;
use parse_text::*;
use std::io::prelude::*;
use md5;


fn main() {
    let file = open("/Users/yoavnir/Documents/vs code/rust/old/solving_challenges/advent_of_code/text.txt");

    let mut i = 0;

    loop {
        let digest = md5::compute(format!("bgvyzdsv{i}"));
        //println!("{:?}",&digest[0..5]);
        if format!("{:x}",&digest)[0..5] == *"00000" {
            break
        }
        i+=1;
    }

    println!("{i}");
    /*
    for line in file_to_iter(file).lines() {
        let line = match line {
            Ok(line) => line,
            Err(error) => panic!()
        };
    }
    */
}