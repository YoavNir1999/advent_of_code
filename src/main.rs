mod parse_text;
use parse_text::*;
use std::io::prelude::*;
use regex::Regex;
use regex::RegexSet;

fn main() {
    let file = open("/Users/yoavnir/Documents/vs code/rust/old/solving_challenges/advent_of_code/text.txt");
    let lines = file_to_iter(file);
    
    for line in lines.lines() {
        let line = match line {
            Ok(line) => line,
            Err(err) => panic!("{}",err)
        };
        let sum : i64 = parse_json(line).iter().sum();
        println!("{}",sum);

    }

}

fn parse_json(line:String) -> Vec<i64> {
    let re = Regex::new(r"-?\d+").unwrap();
    let caps : Vec<i64> = re.find_iter(&line).filter_map(|x| x.as_str().parse::<i64>().ok()).collect();
    
    return caps
}