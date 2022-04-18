mod parse_text;
use parse_text::*;
use std::io::prelude::*;
use md5;


fn main() {
    let file = open("/home/yoavn/Documents/rust/cool stuff/advent_of_code/text.txt");

    let mut arr = vec![[0;1000];1000];

    let mut i = 0;

    loop {
        let digest = md5::compute(format!("bgvyzdsv{i}"));
        //println!("{:?}",&digest[0..5]);
        if format!("{:x}",&digest)[0..5] == *"00000" {
            break
        }
        i+=1;
    }

    println!("{i}");
}