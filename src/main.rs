mod parse_text;
use parse_text::*;
use std::io::prelude::*;

fn main() {
    let file = open("/Users/yoavnir/Documents/vs code/rust/old/solving_challenges/advent_of_code/text.txt");
    let lines = file_to_iter(file);
    
    for line in lines.lines() {
        let line = match line {
            Ok(line) => line,
            Err(err) => panic!("{}",err)
        };
        //let test : Vec<String> = "g,h,j,a,a,b,c,c".split(",").map(|x| x.to_owned()).collect();
        println!("{}",next_pass(line))

    }


}


fn next_pass(line:String) -> String {
    let abc : Vec<String> = "a,b,c,d,e,f,g,h,i,j,k,l,m,n,o,p,q,r,s,t,u,v,w,x,y,z".split(",").map(|x| x.to_owned()).collect();
    let mut line : Vec<String> = line.split(",").map(|x| x.to_owned()).collect();
    loop {
        advance_char(&mut line, &abc);
        //println!("{:?}",line);
        if check_pass(&line) {
            break
        }
    }
    return line.join("")
}


fn advance_char(line:&mut Vec<String>,abc:&Vec<String>) {
    let mut idx = line.len()-1;
    loop {
        if line[idx] != "z" {
            let abc_idx = find_idx(abc, &line[idx]);
            line[idx] = abc[abc_idx+1].clone();
            return
        } else {
            line[idx] ="a".to_owned();
            idx-=1
        }
    }
}


fn check_pass(line:&Vec<String>) -> bool {
    let abc : Vec<String> = "a,b,c,d,e,f,g,h,i,j,k,l,m,n,o,p,q,r,s,t,u,v,w,x,y,z".split(",").map(|x| x.to_owned()).collect();
    let mut three_follow = false;
    let mut first_consec = false;
    let mut second_consec = false;
    let mut pair = ["a".to_owned(),"b".to_owned()];
    for idx in 0..line.len() {
        if "iol".contains(&line[idx]) {
            return false
        }
        // consec checks
        if idx < line.len()-1 {
            if line[idx] == line[idx+1] && first_consec == false {
                pair = [line[idx].clone(),line[idx+1].clone()];
                first_consec=true;
            } else if line[idx] == line[idx+1] && first_consec == true {
                if pair[0]!=line[idx] && pair[1]!=line[idx]  {
                    second_consec=true
                }
            }
        }
        //three follow check
        if idx < line.len()-2 {
            let abc_index = find_idx(&abc, &line[idx]);
            if abc_index < 24 {
                if line[idx+1]==abc[abc_index+1]&&line[idx+2]==abc[abc_index+2] {
                    three_follow = true
                }
            }
        }
    }

    return three_follow && first_consec && second_consec
}

fn find_idx(abc:&Vec<String>,char:&String) -> usize {
    for idx in 0..abc.len() {
        if &abc[idx] == char {
            return idx
        }
    }
    panic!()
}