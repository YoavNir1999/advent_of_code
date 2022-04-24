mod parse_text;
use parse_text::*;
use std::io::prelude::*;

fn main() {
    let file = open("/Users/yoavnir/Documents/vs code/rust/old/solving_challenges/advent_of_code/text.txt");

    let mut board = Board::new();
    loop {
        let file = open("/Users/yoavnir/Documents/vs code/rust/old/solving_challenges/advent_of_code/text.txt");
        let lines = file_to_iter(file);
        for line in lines.lines() {
            let line = match line {
                Ok(line) => line,
                Err(err) => panic!("{}",err)
            };

            match parse_line(line, &mut board) {
                (Some(val),Some(name)) => board.ports.push(Port{name:name,num:Some(val)}),
                _ => continue
            };
        }
        match board.check_val("a") {
            Some(val) => {println!("{val}");break},
            _ => continue
        }
    }
    
}

fn parse_line(line:String,board:&mut Board) -> (Option<u16>,Option<String>) {
    let words : Vec<&str> = line.split(" ").collect();
    if words[1] == "->" {
        if words[0].trim().parse::<u16>().is_ok() {
            return (Some(words[0].trim().parse::<u16>().unwrap()),Some(words[2].to_owned()))
        } else {
            match board.check_val(words[0]) {
                Some(val) => return (Some(val),Some(words[2].to_owned())),
                None => return (None,None)
            }
        }
    } else if words[1] == "RSHIFT" {
        match board.check_val(words[0]) {
            Some(val) => return (Some(Port{name:"a".to_owned(),num:Some(val)}.rshift(words[2].parse::<u16>().unwrap())),Some(words[4].to_owned())),
            None => return (None,None)
        }
    } else if words[1] == "LSHIFT" {
        match board.check_val(words[0]) {
            Some(val) => return (Some(Port{name:"a".to_owned(),num:Some(val)}.lshift(words[2].parse::<u16>().unwrap())),Some(words[4].to_owned())),
            None => return (None,None)
        }
    } else if words[1] == "OR" {
        match (board.check_val(words[0]),board.check_val(words[2])) {
            (Some(val1),Some(val2)) => return (Some(Port{name:"a".to_owned(),num:Some(val1)}.or(Port{name:"b".to_owned(),num:Some(val2)})),Some(words[4].to_owned())),
            _ => return (None,None)
        }
    } else if words[1] == "AND" {
        if words[0].trim().parse::<u16>().is_ok() && board.check_val(words[2]) != None{
            return (Some(Port{name:"a".to_owned(),num:Some(words[0].parse::<u16>().unwrap())}.and(Port{name:"b".to_owned(),num:board.check_val(words[2])})),Some(words[4].to_owned()))
        } else {
            match (board.check_val(words[0]),board.check_val(words[2])) {
                (Some(val1),Some(val2)) => return (Some(Port{name:"a".to_owned(),num:Some(val1)}.and(Port{name:"b".to_owned(),num:Some(val2)})),Some(words[4].to_owned())),
                _ => return (None,None)
            }
        }
    } else if words[0] == "NOT" {
        match board.check_val(words[1]) {
            Some(val) => return (Some(!val),Some(words[3].to_owned())),
            _ => return (None,None)
        }
    }


    return (None,None)
}

struct Port {
    name : String,
    num : Option<u16>
}

struct Board {
    ports : Vec<Port>
}



impl Port {
    fn and(&self,other:Port) -> u16 {
        return self.num.unwrap()&other.num.unwrap()
    }
    fn or(&self,other:Port) -> u16 {
        return self.num.unwrap()|other.num.unwrap()
    }
    fn lshift(&self,num:u16) -> u16 {
        return self.num.unwrap()<<num
    }
    fn rshift(&self,num:u16) -> u16 {
        return self.num.unwrap()>>num
    }
}

impl Board {
    fn new() -> Board {
        return Board{ports:Vec::new()}
    }
    fn check_val(&self,str:&str) -> Option<u16> {
        for port in &self.ports {
            if port.name == str {
                return Some(port.num.unwrap())
            }
        }
        return None
    }
}