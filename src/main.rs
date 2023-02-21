mod parse_text;
use parse_text::*;
use std::{io::prelude::*};
//use std::ops::Add;

fn main() {
    let file = open("/Users/yoavnir/Documents/vs code/rust/old/solving_challenges/advent_of_code/text.txt");
    let lines = file_to_iter(file);

    let mut grid : Vec<Vec<u8>> = Vec::new();

    for line in lines.lines() {
        let line = match line {
            Ok(line) => line,
            Err(err) => panic!("{}",err)
        };

        parse_line(line, &mut grid);
    };

    let mut biggest = 0;

    for i in 1..98 {
        for j in 1..98 {
            let temp = check_vis(&grid,i,j);
            if temp > biggest {
                biggest = temp
            }
        }
    }

    println!("{}",biggest);
}

fn check_vis(grid : &Vec<Vec<u8>>, i : usize, j : usize) -> u32 {
    let val = grid[i][j];

    let mut vals = [0,0,0,0];

    for s in 1..i+1 {
        if grid[i-s][j] >= val {
            vals[0] += 1;
            break
        } else {
            vals[0] += 1
        }
    }

    for s in 1..99-i {
        if grid[i+s][j] >= val {
            vals[1] += 1;
            break
        } else {
            vals[1] += 1
        }
    }

    for s in 1..99-j {
        if grid[i][s+j] >= val {
            vals[2] += 1;
            break
        } else {
            vals[2] += 1
        }
    }

    for s in 1..j+1 {
        if grid[i][j-s] >= val {
            vals[3] += 1;
            break
        } else {
            vals[3] += 1
        }
    }

    let mut res = 1;

    for v in vals {
        res *= v
    }
    res
}

fn parse_line(line:String, grid : &mut Vec<Vec<u8>>) {
    let mut line : Vec<String> = line.split("").map(|x| x.to_owned()).collect();
    line.pop();
    line.remove(0);
    let line : Vec<u8> = line.iter().map(|x| x.parse().unwrap()).collect();
    grid.push(line);
    //println!("{:?}",line);
}