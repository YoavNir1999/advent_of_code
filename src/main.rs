mod parse_text;
use parse_text::*;
use std::io::prelude::*;
//use std::ops::Add;

fn main() {
    let file = open("/Users/yoavnir/Documents/vs code/rust/old/solving_challenges/advent_of_code/text.txt");
    let lines = file_to_iter(file);
    let mut boxes : Vec<u16> = Vec::new();

    for line in lines.lines() {
        let line = match line {
            Ok(line) => line,
            Err(err) => panic!("{}",err)
        };

        parse_line(line,&mut boxes)
    }

    let mut solutions : Vec<Vec<usize>> = Vec::new();
    let mut solution : Vec<usize> = Vec::new();

    solve_recursive(&mut solutions, &boxes,&mut solution,0);

    for sol in &mut solutions {
        sol.sort();
    }

    solutions.dedup();

    solutions.sort_by_key(|x| x.len());

    let mut solutions_len = Vec::new();

    for sol in &mut solutions {
        solutions_len.push(sol.len())
    }

    let shortest = solutions_len[0];

    solutions_len = solutions_len.iter().filter(|x| *x==&shortest).map(|x| *x).collect();

    println!("{:?}",&solutions_len.len());
}

fn parse_line(line:String,boxes:&mut Vec<u16>) {
    //let line : Vec<String> = line.split(" ").map(|x| x.to_owned()).collect();
    let num : u16= line.parse().unwrap();
    boxes.push(num)
}

fn solve_recursive(solutions : &mut Vec<Vec<usize>>,boxes : &Vec<u16>, solution :&mut Vec<usize>,curr_idx:usize) {
    let sum = sum_idx(boxes, solution);
    if sum > 150 {
        return
    } else if sum == 150 {
        solutions.push(solution.clone());
        return
    }
    else {
        for idx in curr_idx..20 {
            if !solution.contains(&idx) {
                solution.push(idx);
                solve_recursive(solutions, boxes, solution,idx);
                solution.pop();
            }
        }
    }
    //println!("{:?}",solutions);
}

fn sum_idx(boxes:&Vec<u16>,solution:&Vec<usize>) -> u16 {
    let mut ans = 0;
    for idx in solution {
        ans+=boxes[*idx]
    }
    return ans
}