use regex::Regex;
use std::fs;

const BAG_RED: i32 = 12;
const BAG_GREEN: i32 = 13;
const BAG_BLUE: i32 = 14;

fn main() {
    //^
    let r = Regex::new(
        r"Game (?<game>\d+): |(((?<red>\d+) red|(?<blue>\d+) blue|(?<green>\d+) green).{0,2})*",
    )
    .unwrap();

    let filename = std::env::args()
        .nth(1)
        .expect("Pass filename as command line argument");

    println!("Filename: {}", filename);

    let input = fs::read_to_string(filename).expect("Unable to read file");

    let mut game_tot = 0;
    let mut power_tot = 0;

    for line in input.lines() {
        let mut game = 0;
        let mut red_max = 0;
        let mut green_max = 0;
        let mut blue_max = 0;

        for ss in line.split(';') {
            for caps in r.captures_iter(ss) {
                if let Some(gm) = &caps.name("game") {
                    let x = gm.as_str();
                    //println!("Game {}", x);
                    game = x.parse::<i32>().expect("parsing game");
                }
                if let Some(r) = &caps.name("red") {
                    let x = r.as_str();
                    //println!("Red {}", x);
                    let v = x.parse::<i32>().expect("parsing red");
                    if red_max < v {
                        red_max = v;
                    }
                }
                if let Some(b) = &caps.name("blue") {
                    let x = b.as_str();
                    //println!("Blue {}", x);
                    let v = x.parse::<i32>().expect("parsing blue");
                    if blue_max < v {
                        blue_max = v;
                    }
                }
                if let Some(g) = &caps.name("green") {
                    let x = g.as_str();
                    //println!("Green {}", x);
                    let v = x.parse::<i32>().expect("parsing green");
                    if green_max < v {
                        green_max = v;
                    }
                }
            }
        } // ss
        if red_max <= BAG_RED && blue_max <= BAG_BLUE && green_max <= BAG_GREEN {
            game_tot += game;
        }
        power_tot += red_max * blue_max * green_max;
    } // line

    println!("Total: {game_tot}, power: {power_tot}");
}
