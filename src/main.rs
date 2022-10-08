mod parse_text;
use parse_text::*;
use std::{io::prelude::*};
//use std::ops::Add;

fn main() {
    let file = open("/Users/yoavnir/Documents/vs code/rust/old/solving_challenges/advent_of_code/text.txt");
    let lines = file_to_iter(file);

    let mut packages : Vec<u64> = Vec::new();

    let mut best = (100,1000000000);

    for line in lines.lines() {
        let line = match line {
            Ok(line) => line,
            Err(err) => panic!("{}",err)
        };
        packages.push(parse_line(line))
    }

    let group_weight : u64 = packages.iter().sum::<u64>()/3;
    let mut main : Vec<u64> = Vec::new();

    recursive_add(&mut packages, 0,&mut best,group_weight,&mut main);

    println!("{:?}",packages);

    /*
    println!("{:?}",configs);

    let res = configs.best_config();
    println!("{res}")
    */


}

fn recursive_add(packages :&Vec<u64>,idx : usize, best : &mut (usize,u64),weight : u64, main : &mut Vec<u64>) {
    if idx < packages.len() {
        let current = packages[idx];
        let idx = idx + 1;
        // include current
        main.push(current);
        recursive_add(packages, idx, best, weight, main);
        // don't include current
        main.pop();
        recursive_add(packages, idx, best, weight, main);

    } else if main.iter().sum::<u64>() == weight {
        let length = main.len();
        let qe : u64 = main.iter().product();
        if length < best.0 {
            *best = (length,qe);
            println!("{:?}",best);
        } else if length == best.0 && qe < best.1 {
            *best = (length,qe);
            println!("{:?}",best);
        }
    }
    

}

fn parse_line(line:String) -> u64 {
    return line.parse().unwrap()
}