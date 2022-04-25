mod parse_text;
use parse_text::*;
use std::io::prelude::*;
use itertools::Itertools;

fn main() {
    let file = open("/Users/yoavnir/Documents/vs code/rust/old/solving_challenges/advent_of_code/text.txt");
    let lines = file_to_iter(file);
    let mut cities : Vec<City> = Vec::new();
    
    for line in lines.lines() {
        let line = match line {
            Ok(line) => line,
            Err(err) => panic!("{}",err)
        };
        let vals = line_to_vec(line);
        let forward = cities.exists(&vals[0]);
        let back = cities.exists(&vals[1]);
        if forward == None {
            cities.push(City {
                name:vals[0].to_owned(),
                paths:vec!(Path {destination:vals[1].to_owned(),distance:vals[2].parse::<u16>().unwrap()})
            })
        } else {
            cities[forward.unwrap()].paths.push(Path {destination:vals[1].to_owned(),distance:vals[2].parse::<u16>().unwrap()})
        }
        if back == None {
            cities.push(City {
                name:vals[1].to_owned(),
                paths:vec!(Path {destination:vals[0].to_owned(),distance:vals[2].parse::<u16>().unwrap()})
            })
        } else {
            cities[back.unwrap()].paths.push(Path {destination:vals[0].to_owned(),distance:vals[2].parse::<u16>().unwrap()})
        }


    }


    /*
    for city in cities {
        for path in city.paths {
            println!("from {} to {} distance {}",city.name,path.destination,path.distance)
        }
    }
    */

    let mut shortest : u32 = 0;

    for city1 in &cities {
        for city2 in &cities {
            for city3 in &cities {
                for city4 in &cities {
                    for city5 in &cities {
                        for city6 in &cities {
                            for city7 in &cities {
                                for city8 in &cities {
                                    let all = vec!(city1,city2,city3,city4,city5,city6,city7,city8);
                                    if all.iter().unique().collect_vec().len() == 8 {
                                        let dist : u32 =  (dist_to(city1, city2) +
                                                    dist_to(city2, city3) +
                                                    dist_to(city3, city4) +
                                                    dist_to(city4, city5) +
                                                    dist_to(city5, city6) +
                                                    dist_to(city6, city7) +
                                                    dist_to(city7, city8)) as u32;
                                        if dist > shortest {
                                            shortest = dist
                                        }
                                    } 
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    println!("{shortest}")

    
}

fn dist_to(from:&City,to:&City) -> u16 {
    for path in &from.paths {
        if path.destination == to.name {
            return path.distance
        }
    }
    panic!()
}

fn line_to_vec(line:String) -> Vec<String> {
    let splited: Vec<&str> = line.split(" ").collect();
    return vec!(splited[0].to_owned(),splited[2].to_owned(),splited[4].to_owned())
}

#[derive(Debug,Hash)]
struct Path {
    destination : String,
    distance : u16
}

#[derive(Hash)]
struct City {
    name : String,
    paths : Vec<Path>
}

trait Exists {
    fn exists(&self,name:&String) -> Option<usize>;
}

impl Exists for Vec<City> {
    fn exists(&self,name:&String) -> Option<usize> {
        let mut idx = 0;
        for city in self {
            if &city.name == name {
                return Some(idx)
            }
            idx += 1;
        }
        return None
    }
}

impl PartialEq for City {
    fn eq(&self,other:&City) -> bool {
        return self.name == other.name
    }
}

impl Eq for City {

}

