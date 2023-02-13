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

        score += determine_score(parse_line(line))
    };

    println!("{}",score);

}

fn parse_line(line:String) -> Turn {
    let line : Vec<&str> = line.split(" ").collect();

    let res = Turn {
        them : infer(line[0]),
        you : infer(line[1])
    };

    //println!("{:?}",res);

    res
}

#[derive(Debug)]
struct Turn {
    them : Choice,
    you : Choice
}

#[derive(Debug)]
enum Choice {
    Rock,
    Paper,
    Scissors
}

fn infer(letter : &str) -> Choice {
    if letter == "A" || letter == "X"{
        return Choice::Rock;
    } else if letter == "B" || letter == "Y" {
        return Choice::Paper
    } else {
        return  Choice::Scissors;
    }
}

fn determine_score(turn : Turn) -> u32 {
    let mut sum : u32 = 0;

    // base score
    match turn.you {
        Choice::Rock => {
                        match turn.them {
                            Choice::Rock => sum += 4,
                            Choice::Scissors => sum += 7,
                            _ => sum += 1
                        }},
        Choice::Paper => {
                        match turn.them {
                            Choice::Rock => sum += 8,
                            Choice::Scissors => sum += 2,
                            _ => sum += 5
                        }},
        Choice::Scissors => {
                        match turn.them {
                            Choice::Rock => sum += 3,
                            Choice::Scissors => sum += 6,
                            _ => sum += 9
                        }}
    }

    sum
}