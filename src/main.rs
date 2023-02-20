mod parse_text;
use parse_text::*;
use std::{io::prelude::*};
//use std::ops::Add;

fn main() {
    let file = open("/Users/yoavnir/Documents/vs code/rust/old/solving_challenges/advent_of_code/text.txt");
    let lines = file_to_iter(file);

    let mut grid : Vec<Vec<u8>> = Vec::new();

    let mut sum : u32 = 99*2+97*2;

    for line in lines.lines() {
        let line = match line {
            Ok(line) => line,
            Err(err) => panic!("{}",err)
        };

        parse_line(line, &mut grid);
    };

    for i in 1..98 {
        for j in 1..98 {
            if check_vis(&grid,i,j) {
                sum += 1
            }
        }
    }

    println!("{}",sum);
}

fn check_vis(grid : &Vec<Vec<u8>>, i : usize, j : usize) -> bool {
    let val = grid[i][j];

    let mut bools = [true,true,true,true];

    for s in 0..i {
        if grid[s][j] >= val {
            bools[0] = false;
            break
        }
    }

    for s in i+1..99 {
        if grid[s][j] >= val {
            bools[1] = false;
            break
        }
    }

    for s in j+1..99 {
        if grid[i][s] >= val {
            bools[2] = false;
            break
        }
    }

    for s in 0..j {
        if grid[i][s] >= val {
            bools[3] = false;
            break
        }
    }

    for v in bools {
        if v == true {
            return true
        }
    }
    false
}

fn parse_line(line:String, grid : &mut Vec<Vec<u8>>) {
    let mut line : Vec<String> = line.split("").map(|x| x.to_owned()).collect();
    line.pop();
    line.remove(0);
    let line : Vec<u8> = line.iter().map(|x| x.parse().unwrap()).collect();
    grid.push(line);
    //println!("{:?}",line);
}