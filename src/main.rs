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
    for _second in 0..2503 {
        for deer in &mut deers {
            deer.counter+=1;
            if deer.counter <= deer.duration {
                deer.location+=deer.speed
            } else if deer.counter==deer.duration+deer.rest {
                deer.counter=0
            }
        }
        let mut max_dist = 0;
        for deer in &deers {
            if deer.location>max_dist {
                max_dist=deer.location
            }
        }
        for deer in &mut deers {
            if deer.location==max_dist {
                deer.score+=1
            }
        }
    }
    for deer in &deers {
        println!("{}",deer.score)
    }

}

#[derive(Debug,Clone)]
struct Deer {
    name : String,
    speed : u32,
    duration : u32,
    rest: u32,
    location : u32,
    counter : u32,
    score : u32
}

fn parse_line(line:String,arr:&mut Vec<Deer>) {
    let line : Vec<String> = line.split(" ").map(|x| x.to_owned()).collect();
    arr.push(
        Deer {
            name : line[0].clone(),
            speed : line[3].parse::<u32>().unwrap(),
            duration : line[6].parse::<u32>().unwrap(),
            location : 0,
            rest : line[13].parse::<u32>().unwrap(),
            counter : 0,
            score : 0
        }
    )
}