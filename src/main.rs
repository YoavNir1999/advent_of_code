mod parse_text;
use parse_text::*;
use std::io::prelude::*;

fn main() {
    let file = open("/Users/yoavnir/Documents/vs code/rust/old/solving_challenges/advent_of_code/text.txt");
    let lines = file_to_iter(file);
    
    for line in lines.lines() {
        let mut line = match line {
            Ok(line) => line,
            Err(err) => panic!("{}",err)
        };
        for _i in 0..50 {
            line = line_to_new_line(line)
        }
        println!("{}",line.len())

    }
}

fn line_to_new_line(line:String) -> String  {
    let mut last = line.chars().next().unwrap();
    let mut new_line = String::new();
    let mut count = 0;
    for char in line.chars() {
        if char == last {
            count+=1
        } else {
            new_line += &format!("{count}{last}");
            last = char;
            count = 1;
        }
    }
    new_line += &format!("{count}{last}");
    return new_line
}