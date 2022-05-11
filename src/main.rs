mod parse_text;
use parse_text::*;
use std::io::prelude::*;
//use std::ops::Add;

fn main() {
    let file = open("/Users/yoavnir/Documents/vs code/rust/old/solving_challenges/advent_of_code/text.txt");
    let lines = file_to_iter(file);
    let mut deers:Vec<Deer> = Vec::new();
    for line in lines.lines() {
        let line = match line {
            Ok(line) => line,
            Err(err) => panic!("{}",err)
        };

        parse_line(line,&mut deers)
    }
    let mut biggest = 0;
    for mut deer in deers {
        let mut remainder = 2503 % deer.cycle;
        if remainder > deer.duration {
            remainder = deer.duration;
        }
        deer.location = (2503.0/deer.cycle as f64).floor() as u32 * deer.speed * deer.duration + remainder * deer.speed;
        if deer.location>biggest {
            biggest=deer.location
        }
        println!("{:?}",deer);
    }
    println!("{:?}",biggest)

}

#[derive(Debug,Clone)]
struct Deer {
    name : String,
    speed : u32,
    duration : u32,
    location : u32,
    cycle : u32
}

fn parse_line(line:String,arr:&mut Vec<Deer>) {
    let line : Vec<String> = line.split(" ").map(|x| x.to_owned()).collect();
    arr.push(
        Deer {
            name : line[0].clone(),
            speed : line[3].parse::<u32>().unwrap(),
            duration : line[6].parse::<u32>().unwrap(),
            location : 0,
            cycle : line[6].parse::<u32>().unwrap()+line[13].parse::<u32>().unwrap()
        }
    )
}