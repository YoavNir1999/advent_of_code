mod parse_text;
use parse_text::*;
use std::{io::prelude::*};
//use std::ops::Add;

fn main() {
    let file = open("/Users/yoavnir/Documents/vs code/rust/old/solving_challenges/advent_of_code/text.txt");
    let lines = file_to_iter(file);

    /*
    [V]     [B]                     [F]
    [N] [Q] [W]                 [R] [B]
    [F] [D] [S]     [B]         [L] [P]
    [S] [J] [C]     [F] [C]     [D] [G]
    [M] [M] [H] [L] [P] [N]     [P] [V]
    [P] [L] [D] [C] [T] [Q] [R] [S] [J]
    [H] [R] [Q] [S] [V] [R] [V] [Z] [S]
    [J] [S] [N] [R] [M] [T] [G] [C] [D]
    1   2   3   4   5   6   7   8   9 
     */

    let mut stack : Vec<Vec<&str>> = vec!(
        vec!("V","N","F","S","M","P","H","J"),
        vec!("Q","D","J","M","L","R","S"),
        vec!("B","W","S","C","H","D","Q","N"),
        vec!("L","C","S","R"),
        vec!("B","F","P","T","V","M"),
        vec!("C","N","Q","R","T"),
        vec!("R","V","G"),
        vec!("R","L","D","P","S","Z","C"),
        vec!("F","B","P","G","V","J","S","D")
    );

    for line in lines.lines() {
        let line = match line {
            Ok(line) => line,
            Err(err) => panic!("{}",err)
        };

        parse_line(line, &mut stack);
    };
    
    for st in stack {
        print!("{}", st[0]);
    }

}

fn perform_order(stack : &mut Vec<Vec<&str>>,order : Order) {
    println!("{:?}",order);
    println!("{:?}",stack[order.from]);
    println!("{:?}",stack[order.to]);
    for i in 0..order.amount {
        let element = stack[order.from].remove(0);
        stack[order.to].insert(0, element);
    }
    println!("{:?}",stack[order.from]);
    println!("{:?}",stack[order.to]);
}


fn parse_line(line:String, stack : &mut Vec<Vec<&str>>) {
    let line : Vec<&str> = line.split(" ").collect();
    let order = Order {
        amount : line[1].parse::<usize>().unwrap(),
        from : line[3].parse::<usize>().unwrap()-1,
        to : line[5].parse::<usize>().unwrap()-1,
    };
    
    perform_order(stack, order);

}

#[derive(Debug)]
struct Order {
    amount : usize,
    from : usize,
    to : usize
}