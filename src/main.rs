mod parse_text;
use parse_text::*;
use std::{io::prelude::*};
//use std::ops::Add;

fn main() {
    let file = open("/Users/yoavnir/Documents/vs code/rust/old/solving_challenges/advent_of_code/text.txt");
    let lines = file_to_iter(file);

    let mut score : u32 = 0;

    for line in lines.lines() {
        let line = match line {
            Ok(line) => line,
            Err(err) => panic!("{}",err)
        };

        if parse_line(line) {
            score += 1
        }
    };

    println!("{}",score);

}


fn parse_line(line:String) -> bool {
    let line : Vec<Vec<u8>> = line.split(",").map(|x| x.split("-").map(|x| x.parse().unwrap()).collect()).collect();
    //println!("{:?}",line);
    if line[0][0] <= line[1][0] && line[0][1] >= line[1][1] || line[1][0] <= line[0][0] && line[1][1] >= line[0][1] {
        return true
    }


    false
}
