mod parse_text;
use parse_text::*;
use std::io::prelude::*;
use std::ops::Add;

fn main() {
    let file = open("/Users/yoavnir/Documents/vs code/rust/old/solving_challenges/advent_of_code/text.txt");
    let lines = file_to_iter(file);
    let mut seaters:Vec<SeatingInfo> = Vec::new();
    for line in lines.lines() {
        let line = match line {
            Ok(line) => line,
            Err(err) => panic!("{}",err)
        };

        parse_line(line,&mut seaters)
    }
    for seater in &seaters {
        println!("{:?}\n",seater);
    }

    //println!("{}",&seaters[0]+&seaters[3]);
    let mut answers : Vec<i32> = Vec::new();
    let mut new : Vec<SeatingInfo> = Vec::new();

    permutate(&mut seaters, &mut answers,&mut new);
    answers.sort();
    println!("{:?}",answers.last().unwrap());
}

fn permutate(arr:&Vec<SeatingInfo>,answers:&mut Vec<i32>,new:&mut Vec<SeatingInfo>) {
    if new.len()==8 {
        answers.push(new.compute());
    } else {
        for sit in arr {
            if !new.contains(sit) {
                new.push(sit.clone());
                permutate(arr, answers, new);
                new.pop();
            }
        }
    }
}

#[derive(Debug,Clone)]
struct SeatingInfo {
    person1 : String,
    person2 : Vec<String>,
    number : Vec<i32>
}

fn parse_line(line:String,seaters:&mut Vec<SeatingInfo>) {
    let line : Vec<String> = line.split(" ").map(|x| x.to_owned()).collect();
    let num : i32;
    if line[2] == "gain" {
        num = line[3].parse().unwrap();
    } else {
        num = line[3].parse::<i32>().unwrap()*-1;
    }
    if seaters.is_empty() {
        seaters.push(SeatingInfo {
            person1 : line[0].clone(),
            person2 : vec!(line[10].clone()),
            number : vec!(num)
    })} else if seaters.last().unwrap().person2.len() == 7 {
        seaters.push(SeatingInfo {
            person1 : line[0].clone(),
            person2 : vec!(line[10].clone()),
            number : vec!(num)
    });} else {
        seaters.last_mut().unwrap().person2.push(line[10].clone());
        seaters.last_mut().unwrap().number.push(num)
    }
}

trait compute_total {
    fn compute(&self) -> i32;
}

impl compute_total for Vec<SeatingInfo> {
    fn compute(&self) -> i32 {
        let len = self.len();
        let mut total = 0;
        for sit in 0..len-1 {
            total += &self[sit]+&self[sit+1]
        }
        total += &self[len-1]+&self[0];
        total
    }
}

impl PartialEq for SeatingInfo {
    fn eq(&self,other:&SeatingInfo) -> bool {
        self.person1==other.person1
    }
}

impl Add for &SeatingInfo {
    type Output = i32;

    fn add(self,other:&SeatingInfo) -> i32 {
        let name1 = &self.person1;
        let name2 = &other.person1;
        let mut num1 : i32 = 0;
        let mut num2 : i32 = 0;
        for idx in 0..7 {
            if &self.person2[idx]==name2 {
                num1 = self.number[idx]
            } if &other.person2[idx]==name1 {
                num2 = other.number[idx]
            }
            if num1 != 0 && num2 != 0 {
                break
            }
        }
        return num1+num2
    }
}