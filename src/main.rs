mod parse_text;
use parse_text::*;
use std::{io::prelude::*};
//use std::ops::Add;

fn main() {
    let file = open("/Users/yoavnir/Documents/vs code/rust/old/solving_challenges/advent_of_code/text.txt");
    let lines = file_to_iter(file);

    let mut highest = [0,0,0];
    let mut current : i32 = 0;

    for line in lines.lines() {
        let line = match line {
            Ok(line) => line,
            Err(err) => panic!("{}",err)
        };

        if line.is_empty() {
            if current > highest[0] {
                highest[2] = highest[1];
                highest[1] = highest[0];
                highest[0] = current;
            } else if current > highest[1] {
                highest[2] = highest[1];
                highest[1] = current
            } else if current > highest[2] {
                highest[2] = current;
            }
            current = 0;
        } else {
            current += line.parse::<i32>().unwrap()
        }
        
    }

    println!("{}",highest[0]+highest[1]+highest[2]);

}

fn parse_line(line:String) -> u64 {
    return line.parse().unwrap()
}