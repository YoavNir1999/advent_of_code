mod parse_text;
use parse_text::*;
use std::io::prelude::*;
//use std::ops::Add;

fn main() {
    let file = open("/Users/yoavnir/Documents/vs code/rust/old/solving_challenges/advent_of_code/text.txt");
    let lines = file_to_iter(file);
    let sender : Vec<(String,u16)> = vec!(("children",3),("cats",7),("samoyeds",2),("pomeranians",3),("akitas",0),("vizslas",0),("goldfish",5),("trees",3),("cars",2),("perfumes",1))
                                        .iter().map(|x| (x.0.to_owned(),x.1)).collect();

    println!("{:?}",sender);

    for line in lines.lines() {
        let line = match line {
            Ok(line) => line,
            Err(err) => panic!("{}",err)
        };

        match parse_line(line,&sender) {
            (true,num) => {println!("{num}");break},
            _ => ()
        }
    }
}

fn parse_line(line:String,sender:&Vec<(String,u16)>) -> (bool,u16) {
    let line : Vec<String> = line.split(" ").map(|x| x.to_owned()).collect();

    let sue_num = line[1].clone().parse().unwrap();

    let exp1 = (line[2].clone(),line[3].clone().parse::<u16>().unwrap());
    let exp2 = (line[4].clone(),line[5].clone().parse::<u16>().unwrap());
    let exp3 = (line[6].clone(),line[7].clone().parse::<u16>().unwrap());
    
    for clue in sender {
        if clue.0 == exp1.0 && clue.1!=exp1.1 {
            return (false,0)
        }
        if clue.0 == exp2.0 && clue.1!=exp2.1 {
            return (false,0)
        }
        if clue.0 == exp3.0 && clue.1!=exp3.1 {
            return (false,0)
        }
    }
    
    (true,sue_num)
}


