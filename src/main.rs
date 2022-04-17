mod parse_text;
use parse_text::*;
use std::io::prelude::*;


fn main() {
    let file = open("/home/yoavn/Documents/rust/cool stuff/advent_of_code/text.txt");

    let mut arr = vec![[0;1000];1000];

    for line in file_to_iter(file).lines() {
        let line = match line {
            Ok(line) => line,
            Err(error) => panic!()
        };

        execute_line(line,&mut arr);
    }

    let mut count = 0;
    for i in arr {
        for j in i {
            count+=j
        }
    }

    println!("{count}")
}

//structs and impls

fn str_to_nums(line:&str) -> [usize;2] {
    let mut idx = 0;
    for char in line.chars() {
        if char == ',' {
            break
        }
        idx+=1
    }
    let num_1:usize = line[0..idx].parse().unwrap();
    let num_2:usize = line[idx+1..].parse().unwrap();
    [num_1,num_2]
}

    fn execute_line(line:String,array : &mut Vec<[i32;1000]>)  {
        let instruction: Vec<&str> = line.split(" ").collect();
        if instruction[0]=="turn" && instruction[1]=="on" {
            let start = str_to_nums(instruction[2]);
            let end = str_to_nums(instruction[4]);
            for i in start[0]..end[0]+1 {
                for j in start[1]..end[1]+1 {
                    array[i][j]+=1
                }
            }
        }

        else if instruction[0]=="turn" && instruction[1]=="off" {
            let start = str_to_nums(instruction[2]);
            let end = str_to_nums(instruction[4]);
            for i in start[0]..end[0]+1 {
                for j in start[1]..end[1]+1 {
                    if array[i][j] > 0 {
                        array[i][j] -= 1
                    }
                }
            }
        }
        
        else if instruction[0]=="toggle" {
            let start = str_to_nums(instruction[1]);
            let end = str_to_nums(instruction[3]);
            for i in start[0]..end[0]+1 {
                for j in start[1]..end[1]+1 {
                    array[i][j] += 2
                }
            }
        }
    }


