mod parse_text;
use parse_text::*;
use std::{io::prelude::*};
//use std::ops::Add;

fn main() {
    let file = open("/Users/yoavnir/Documents/vs code/rust/old/solving_challenges/advent_of_code/text.txt");
    let lines = file_to_iter(file);

    let mut quartets : Vec<Vec<String>> = Vec::new();

    for line in lines.lines() {
        let line = match line {
            Ok(line) => line,
            Err(err) => panic!("{}",err)
        };

        parse_line(line, &mut quartets);

    };

    for i in 0..quartets.len() {
        if check_quart(&quartets[i]) {
            println!("{}",i+14);
            break;
        }
    }

}

fn check_quart(quart : &Vec<String>) -> bool {
    let mut quart = quart.clone();
    quart.sort();
    quart.dedup();
    if quart.len() == 14 {
        return true
    }

    false
}

fn parse_line(line:String, quartets :&mut Vec<Vec<String>>) {
    let mut line : Vec<String> = line.split("").map(|x| x.to_owned()).collect();
    line.remove(0);
    line.pop();
    
    for i in 0..line.len()-13 {
        let mut temp = Vec::new();
        for j in 0..14 {
            temp.push(line[i+j].clone())
        }
        quartets.push(temp)
    }

    //println!("{:?}",quartets);
}
