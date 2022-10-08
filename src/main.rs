mod parse_text;
use parse_text::*;
use std::{io::prelude::*};
//use std::ops::Add;

fn main() {
    let file = open("/Users/yoavnir/Documents/vs code/rust/old/solving_challenges/advent_of_code/text.txt");
    let lines = file_to_iter(file);

    let mut packages : Vec<i32> = Vec::new();
    let mut configs : Vec<Arrangement> = Vec::new();

    let mut best = (100,1000000000);

    let mut arrangement = Arrangement {
        main : Vec::new(),
        cmp1 : Vec::new(),
        cmp2 : Vec::new(),
        last : 0
    };

    for line in lines.lines() {
        let line = match line {
            Ok(line) => line,
            Err(err) => panic!("{}",err)
        };
        packages.push(parse_line(line))
    }

    recursive_add(&mut packages, &mut arrangement, &mut configs,0,&mut best);

    /*
    println!("{:?}",configs);

    let res = configs.best_config();
    println!("{res}")
    */


}

fn recursive_add(packages :&Vec<i32>, arrangement : &mut Arrangement, configs : &mut Vec<Arrangement>,idx : usize,best : &mut (usize,i32)) {
    if packages.len() == idx {
        if arrangement.is_balanced() {
            /*
            configs.push(arrangement.clone());
            //println!("{:?}",arrangement);
            //arrangement.pop();
            */
            
            if arrangement.main.len() < best.0 {
                *best = (arrangement.main.len(),arrangement.compute_qe());
                println!("{:?}",best)
            } else if arrangement.main.len() == best.0 && arrangement.compute_qe() < best.1 {
                *best = (arrangement.main.len(),arrangement.compute_qe());
                println!("{:?}",best)
            }
        }
    }
    else {
        let current = packages[idx];

        for i in 0..3 {
            if i == 2 {
                arrangement.main.push(current);
                arrangement.last = 0;
                //println!("{:?}",arrangement);
                let idx = idx + 1;
                recursive_add(packages, arrangement, configs,idx,best);
                arrangement.main.pop();

            } else if i == 1 {
                arrangement.cmp1.push(current);
                arrangement.last = 1;
                //println!("{:?}",arrangement);
                let idx = idx + 1;
                recursive_add(packages, arrangement, configs,idx,best);
                arrangement.cmp1.pop();

            } else if i == 0 {
                arrangement.cmp2.push(current);
                arrangement.last = 2;
                //println!("{:?}",arrangement);
                let idx = idx + 1;
                recursive_add(packages, arrangement, configs,idx,best);
                arrangement.cmp2.pop();
            }
        }
        
    }
}

#[derive(Clone,Debug)]
struct Arrangement {
    main : Vec<i32>,
    cmp1 : Vec<i32>,
    cmp2 : Vec<i32>,
    last : i32
}

impl Arrangement {
    fn pop(&mut self) -> i32 {
        if self.last == 0 {
            self.main.pop().unwrap()
        } else if self.last == 1 {
            self.cmp1.pop().unwrap()
        } else {
            self.cmp2.pop().unwrap()
        }
    }
}

fn parse_line(line:String) -> i32 {
    return line.parse().unwrap()
}

impl Arrangement {
    fn compute_qe(&self) -> i32 {
        let mut res = 1;
        for package in &self.main {
            res *= package
        }
        res.abs()
    }
    
    fn is_balanced(&self) -> bool {
        if self.main.iter().sum::<i32>() == self.cmp1.iter().sum::<i32>()
        && self.cmp1.iter().sum::<i32>() == self.cmp2.iter().sum::<i32>() {
            return true
        } else {
            return false
        }
    }
}

trait BestConfig {
    fn best_config(&self) -> i32;
}

impl BestConfig for Vec<Arrangement> {
    fn best_config(&self) -> i32 {
        let mut configs : Vec<(usize,i32)> = Vec::new();
        for arrangement in self {
            configs.push((arrangement.main.len(),arrangement.compute_qe()))
        }
        configs.sort_by_key(|x| x.0);
        let minimum = configs[0].0;
        let mut configs : Vec<&(usize,i32)> = configs.iter().filter(|x| x.0==minimum).collect();
        configs.sort_by_key(|x| x.1);
        return configs[0].1
    }
}