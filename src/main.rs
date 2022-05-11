mod parse_text;
use parse_text::*;
use std::io::prelude::*;
//use std::ops::Add;

fn main() {
    let file = open("/Users/yoavnir/Documents/vs code/rust/old/solving_challenges/advent_of_code/text.txt");
    let lines = file_to_iter(file);
    let mut ings:Vec<Ing> = Vec::new();
    for line in lines.lines() {
        let line = match line {
            Ok(line) => line,
            Err(err) => panic!("{}",err)
        };

        parse_line(line,&mut ings)
    }

    let mut biggest = 0;

    for i1 in 0..101 {
        for i2 in 0..101-i1 {
            for i3 in 0..101-i1-i2 {
                let i4=100-i1-i2-i3;
                let calc = calc_comb([i1,i2,i3,i4], &ings);
                if calc>biggest {
                    biggest=calc
                }
            }
        }
    }

    println!("{biggest}")
}

fn calc_comb(comb:[i64;4],ings:&Vec<Ing>) -> i64 {
    let mut c = 0;
    let mut d = 0;
    let mut f = 0;
    let mut t = 0;
    let mut cals = 0;

    for idx in 0..4 {
        c += ings[idx].capacity*comb[idx];
        d += ings[idx].durab*comb[idx];
        f += ings[idx].flav*comb[idx];
        t += ings[idx].texture*comb[idx];
        cals += ings[idx].cals*comb[idx];
    }
    if c < 0 {c=0};
    if d < 0 {d=0};
    if f < 0 {f=0};
    if t < 0 {t=0};


    return c*d*f*t

    
}

#[derive(Debug,Clone)]
struct Ing {
    name : String,
    capacity : i64,
    durab : i64,
    flav : i64,
    texture : i64,
    cals : i64
}

fn parse_line(line:String,arr:&mut Vec<Ing>) {
    let line : Vec<String> = line.split(" ").map(|x| x.to_owned()).collect();
    arr.push(
        Ing {
            name : line[0].clone(),
            capacity : line[2].parse().unwrap(),
            durab :  line[4].parse().unwrap(),
            flav : line[6].parse().unwrap(),
            texture :  line[8].parse().unwrap(),
            cals :  line[10].parse().unwrap(),
        }
    )
}