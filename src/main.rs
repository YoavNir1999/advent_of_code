mod parse_text;
use parse_text::*;
use std::io::prelude::*;


fn main() {
    let file = open("/home/yoavn/Documents/rust/cool stuff/advent_of_code/text.txt");

    let mut board = Board::new();

    for line in file_to_iter(file).lines() {
        let line = match line {
            Ok(line) => line,
            Err(error) => panic!()
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