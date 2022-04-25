mod parse_text;
use parse_text::*;
use std::io::prelude::*;

fn main() {
    let file = open("/Users/yoavnir/Documents/vs code/rust/old/solving_challenges/advent_of_code/text.txt");
    let lines = file_to_iter(file);
    let mut total = 0;
    
    for line in lines.lines() {
        let line = match line {
            Ok(line) => line,
            Err(err) => panic!("{}",err)
        };
        let long = line.len();
        let short = encode(&line).len();
        total += short-long;
    }
    println!("{total}")
}

fn encode(input: &String) -> String {
    let input = input.as_bytes();
    let mut r = String::new();
    r.push('"');
    for c in input.iter() {
        match *c {
            b'"' => r.push_str("\\\""),
            b'\\' => r.push_str("\\\\"),
            _ => r.push(*c as char),
        }
    }
    r.push('"');
    return r;
}

fn decode(input: &String) -> String {
    let input = input.as_bytes();
    let mut r = String::new();
    let mut i = 1;
    while i < input.len() - 1 {
        if input[i] == b'\\' {
            if input[i + 1] == b'\\' {
                r.push('\\');
                i += 2;
            } else if input[i + 1] == b'"' {
                r.push('"');
                i += 2;
            } else if input[i + 1] == b'x' {
                r.push('?'); // I'm being lazy!
                i += 4;
            }
        } else {
            r.push(input[i] as char);
            i += 1;
        }
    }
    return r;
}