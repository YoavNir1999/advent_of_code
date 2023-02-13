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

        score += let_to_num(&parse_line(line));
    };

    println!("{}",score);

}

fn let_to_num(letter : &str) -> u32 {
    if letter.chars().next().unwrap().is_uppercase() {
        return (letter.as_bytes()[0]-38) as u32
    } else {
        return (letter.as_bytes()[0]-96) as u32
    }
}

fn parse_line(line:String) -> String {
    let mut line : Vec<&str> = line.split("").collect();
    line.pop();
    line.remove(0);

    let len = line.len();

    for i in 0..len/2 {
        for j in len/2..len {
            if line[i]==line[j] {
                return line[i].to_owned()
            }
        }
    }

    //println!("{:?}",line);
    "".to_owned()
}
