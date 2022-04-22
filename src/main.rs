mod parse_text;
use parse_text::*;
use std::io::prelude::*;
use md5;


fn main() {
    let file = open("/Users/yoavnir/Documents/vs code/rust/old/solving_challenges/advent_of_code/text.txt");
    let lines = file_to_iter(file);

    let mut board = Board::new();

    for line in lines.lines() {
        let line = match line {
            Ok(line) => line,
            Err(err) => panic!("{}",err)
        };



    }
}

fn parse_line(line:String) -> Line_parse {
    let words : Vec<&str> = line.split(" ").collect();



    
}

struct Port {
    name : &'static str,
    num : i16
}

struct Board {
    ports : Vec<Port>
}

enum op {
    And,
    Or,
    Rshift,
    Lshift,
    Not,
    Num
}

struct Line_parse {
    operation:op,
    num : Option<i16>,
    arg1 : Option<String>,
    arg2 : Option<String> 
}

impl Port {
    fn and(&self,other:Port) -> i16 {
        return self.num&other.num
    }
    fn or(&self,other:Port) -> i16 {
        return self.num|other.num
    }
    fn lshift(&self,num:i16) -> i16 {
        return self.num<<num
    }
    fn rshift(&self,num:i16) -> i16 {
        return self.num>>num
    }
}

impl Board {
    fn new() -> Board {
        return Board{ports:Vec::new()}
    }


}