use std::cmp::{max, min};
use std::{collections::HashSet, fs};

#[derive(Debug, Clone, Copy)]
struct Schematic {
    c: char,
    is_number: bool,
    is_symbol: bool,
    is_adjacent: bool,
    num_num: Option<usize>,
}

fn map_to_nums(map: &mut Vec<Vec<Schematic>>) -> Vec<u32> {
    let mut out: Vec<u32> = vec![]; // = String::new();
    let mut cn = String::new();
    let mut count: usize = 0;
    let mut was_num = false;
    for r in map {
        for s in r {
            if s.is_number && (s.is_adjacent || was_num) {
                //out.push(s.c);
                cn.push(s.c);
                was_num = true;
                s.num_num = Some(count);
            } else if was_num {
                //out.push(' ');
                was_num = false;
                match cn.parse::<u32>() {
                    Ok(x) => {
                        out.push(x);
                        println!("Number {count} = {x}");
                        count += 1;
                    }
                    Err(e) => println!("Parse failed: {cn}: {e}"),
                }
                cn.clear();
            }
        }
        was_num = false;
        match cn.parse::<u32>() {
            Ok(x) => {
                out.push(x);
                println!("Number {count} = {x}");
                count += 1;
            }
            Err(e) => println!("Parse failed: {cn}: {e}"),
        }
        cn.clear();
    }
    out
}

/*
fn sum_of_numbers_in_string(i: String) -> u32 {
    let mut out: u32 = 0;

    for n in i.split_ascii_whitespace() {
        match n.parse::<u32>() {
            Ok(x) => out += x,
            Err(e) => println!("Parse failed: {n}: {e}"),
        }
    }
    out
}*/

fn adjacent_nums(
    map: &Vec<Vec<Schematic>>,
    rows: usize,
    cols: usize,
    r_c: usize,
    c_c: usize,
) -> HashSet<usize> {
    let mut out = HashSet::<usize>::new();

    /*println!(
        "rows: {}..{}, cols: {}..{}",
        max(0, r_c - 1),
        min(rows, r_c + 1),
        max(0, c_c - 1),
        min(cols, c_c + 1)
    );*/

    for r in max(0, r_c - 1)..min(rows, r_c + 2) {
        for c in max(0, c_c - 1)..min(cols, c_c + 2) {
            //println!("check: {r},{c}");
            if let Some(x) = map[r][c].num_num {
                println!("beep: {r},{c}: {x}");
                out.insert(x);
            }
        }
    }
    out
}

fn calculate_gears(map: &Vec<Vec<Schematic>>, nums: Vec<u32>, rows: usize, cols: usize) -> u32 {
    let mut out: u32 = 0;

    for r in 0..rows {
        for c in 0..cols {
            if map[r][c].is_symbol {
                println!("Symbol? {r},{c}");
                let adj_nums = adjacent_nums(map, rows, cols, r, c);
                if adj_nums.len() == 2 {
                    println!("Gear! {r},{c}");
                    let mut i = adj_nums.iter();
                    out += nums[*i.next().expect("First item in adj_nums")]
                        * nums[*i.next().expect("second item in adj_nums")];
                }
            }
        }
    }

    out
}

fn main() {
    let filename = std::env::args()
        .nth(1)
        .expect("Pass filename as command line argument");

    println!("Filename: {}", filename);

    let input = fs::read_to_string(filename).expect("Unable to read file");

    let height = input.lines().next().expect("Zero lines in file").len();
    let mut map: Vec<Vec<Schematic>> = vec![];

    // We kinda rotate the map by 90 degrees here
    for (x, line) in input.lines().enumerate() {
        map.push(vec![
            Schematic {
                c: '.',
                is_number: false,
                is_symbol: false,
                is_adjacent: true,
                num_num: None,
            };
            height
        ]);
        for (y, c) in line.chars().enumerate() {
            map[x][y].c = c;
            map[x][y].is_number = c.is_ascii_digit();
            map[x][y].is_symbol = !map[x][y].is_number && c != '.';
            map[x][y].is_adjacent = map[x][y].is_symbol;

            if map[x][y].is_number {
                if x > 0 {
                    map[x][y].is_adjacent |= map[x - 1][y].is_adjacent;
                    if y < height - 1 {
                        map[x][y].is_adjacent |= map[x - 1][y + 1].is_adjacent;
                    }
                }
                if y > 0 {
                    map[x][y].is_adjacent |= map[x][y - 1].is_adjacent;
                }
                if x > 0 && y > 0 {
                    map[x][y].is_adjacent |= map[x - 1][y - 1].is_symbol;
                }
            }
        }
    }
    let width = map.len();

    for x in (0..(width)).rev() {
        for y in (0..height).rev() {
            if map[x][y].is_number {
                if x < width - 1 {
                    map[x][y].is_adjacent |= map[x + 1][y].is_adjacent;
                    if y > 0 {
                        map[x][y].is_adjacent |= map[x + 1][y - 1].is_adjacent;
                    }
                }
                if y < height - 1 {
                    map[x][y].is_adjacent |= map[x][y + 1].is_adjacent;
                }
                if y < height - 1 && x < width - 1 {
                    map[x][y].is_adjacent |= map[x + 1][y + 1].is_symbol;
                }
            }
        }
    }
    // Map is read and labelled now
    let nums = map_to_nums(&mut map);
    let result: u32 = nums.iter().sum();
    println!("{:#?}", nums);
    /*for l in map {
        let rowstr = map_row_to_nums(&l);
        println!("{}", rowstr);
        result += sum_of_numbers_in_string(rowstr);
    }*/
    println!("Result: {result}");
    let gears = calculate_gears(&map, nums, width, height);
    println!("Gears: {gears}");
}

//wrong: 1265119781
