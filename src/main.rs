mod parse_text;
use parse_text::*;
use std::io::prelude::*;
//use std::ops::Add;

fn main() {
    /*
    let file = open("/Users/yoavnir/Documents/vs code/rust/old/solving_challenges/advent_of_code/text.txt");
    let lines = file_to_iter(file);

    for line in lines.lines() {
        let line = match line {
            Ok(line) => line,
            Err(err) => panic!("{}",err)
        };

    }
    */
    
    let mut player = Player::new();
    let mut boss = Boss {
        health : 103,
        damage : 9,
        armor : 2
    };

    let weapons = [
        [0,0],
        [8,4],
        [10,5],
        [25,6],
        [40,7],
        [74,8]
    ];

    let armors = [
        [0,0],
        [13,1],
        [31,2],
        [53,3],
        [75,4],
        [102,5]
    ];

    let rings: [[i32;3];8] = [
        [0,0,0],
        [0,0,0],
        [25,1,0],
        [50,2,0],
        [100,3,0],
        [20,0,1],
        [40,0,2],
        [80,0,3]
    ];

    let mut cheapest = 200;

    for weapon in weapons {
        for armor in armors {
            for ring1 in 0..8 {
                for ring2 in 0..8 {
                    if ring1 != ring2 {
                        let cost = weapon[0]+armor[0]+rings[ring1][0]+rings[ring2][0];
                        player.armor = armor[1]+rings[ring1][2]+rings[ring2][2];
                        player.damage = weapon[1]+rings[ring1][1]+rings[ring2][1];
                        if game(&player, &boss) && cost < cheapest {
                            cheapest = cost
                        }
                    }
                }
            }
        }
    }

    println!("{cheapest}")
}

fn game(player : &Player, boss : &Boss) -> bool {
    let mut player = player.clone();
    let mut boss = boss.clone();

    loop {
        // player attack
        if player.damage-boss.armor >= 1 {
            boss.health -= player.damage-boss.armor;
        } else {
            boss.health -= 1;
        }
        if boss.health <= 0 {
            return true
        }
        // boss attack
        if boss.damage-player.armor >= 1 {
            player.health -= boss.damage-player.armor;
        } else {
            player.health -= 1;
        }
        if player.health <= 0 {
            return false
        }
    }
}

#[derive(Clone)]
struct Player {
    health : i32,
    armor : i32,
    damage : i32,
    coins : i32,
}

impl Player {
    fn new() -> Player {
        return Player { health: 100, armor: 0, damage: 0, coins: 0 }
    }
}

#[derive(Clone)]
struct Boss {
    health : i32,
    armor : i32,
    damage : i32
}

/*
fn parse_line(line:String) {
    //let line : Vec<String> = line.split(" ").map(|x| x.to_owned()).collect();
}
*/
