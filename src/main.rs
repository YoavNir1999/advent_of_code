mod parse_text;
use parse_text::*;
use std::io::prelude::*;
//use std::ops::Add;

fn main() {
    let file = open("/Users/yoavnir/Documents/vs code/rust/old/solving_challenges/advent_of_code/text.txt");
    let lines = file_to_iter(file);

    let mut instructions : Vec<Instruction> = Vec::new();

    for line in lines.lines() {
        let line = match line {
            Ok(line) => line,
            Err(err) => panic!("{}",err)
        };
        instructions.push(parse_line(line))
    }
    /*
    for ins in &instructions {
        println!("{:?}",ins)
    }
    */
    

    let mut a : i128 = 0;
    let mut b : i128 = 0;

    let mut idx : i128 = 0;

    while idx < instructions.len() as i128{
        match &instructions[idx as usize] {
            Instruction::hlf(reg) => {
                match reg {
                    Register::a => a/=2,
                    Register::b => b/=2
                };
                idx += 1;
            },

            Instruction::inc(reg) => {
                match reg {
                    Register::a => a+=1,
                    Register::b => b+=1
                };
                idx += 1;
            },

            Instruction::jie(reg,jump ) => {
                match reg {
                    Register::a => {
                        if a%2==0 {
                            idx += jump
                        } else {
                            idx += 1
                        }
                    },
                    Register::b => {
                        if b%2==0 {
                            idx += jump
                        } else {
                            idx += 1
                        }
                    },
                }
            },

            Instruction::jio(reg, jump) => {
                match reg {
                    Register::a => {
                        if a==1 {
                            idx += jump
                        } else {
                            idx += 1
                        }
                    },
                    Register::b => {
                        if b==1 {
                            idx += jump
                        } else {
                            idx += 1
                        }
                    },
                }
            },

            Instruction::jmp(jump) => {
                idx += jump
            },

            Instruction::tpl(reg) => {
                match reg {
                    Register::a => a*=3,
                    Register::b => b*=3
                };
                idx += 1;
            }
        }
    }

    println!("{b}");
}

#[derive(Debug)]
enum Register {
    a,
    b
}

#[derive(Debug)]
enum Instruction {
    hlf(Register),
    tpl(Register),
    inc(Register),
    jmp(i128),
    jie(Register,i128),
    jio(Register,i128)
}

fn parse_line(line:String) -> Instruction {
    let line : Vec<&str> = line.split(" ").collect();
    if line[0]=="hlf" {
        if line[1] == "a" {
            return Instruction::hlf(Register::a)
        } else {
            return Instruction::hlf(Register::b)
        }
    } else if line[0]=="tpl" {
        if line[1] == "a" {
            return Instruction::tpl(Register::a)
        } else {
            return Instruction::tpl(Register::b)
        }
    } else if line[0]=="inc" {
        if line[1] == "a" {
            return Instruction::inc(Register::a)
        } else {
            return Instruction::inc(Register::b)
        }
    } else if line[0]=="jmp" {
        return Instruction::jmp(line[1].parse().unwrap())
    } else if line[0]=="jio" {
        if line[1] == "a" {
            return Instruction::jio(Register::a,line[2].parse().unwrap())
        } else {
            return Instruction::jio(Register::b,line[2].parse().unwrap())
        }
    } else if line[0]=="jie" {
        if line[1] == "a" {
            return Instruction::jie(Register::a,line[2].parse().unwrap())
        } else {
            return Instruction::jie(Register::b,line[2].parse().unwrap())
        }
    } else {
        panic!("parse error")
    }
}
