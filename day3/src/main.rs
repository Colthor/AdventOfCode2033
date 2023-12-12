use std::fs;

#[derive(Debug, Clone, Copy)]
struct Schematic {
    c: char,
    is_number: bool,
    is_symbol: bool,
    is_adjacent: bool,
}

fn map_row_to_string(i: &Vec<Schematic>) -> String {
    let mut out = String::new();
    let mut was_num = false;

    for s in i {
        if s.is_number && (s.is_adjacent || was_num) {
            out.push(s.c);
            was_num = true;
        } else if was_num {
            out.push(' ');
            was_num = false;
        }
    }
    out
}

fn sum_of_numbers_in_string(i: String) -> u32 {
    let mut out: u32 = 0;

    for n in i.split_ascii_whitespace() {
        match n.parse::<u32>() {
            Ok(x) => out += x,
            Err(e) => println!("Parse failed: {n}: {e}"),
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
    let mut result: u32 = 0;
    for l in map {
        let rowstr = map_row_to_string(&l);
        println!("{}", rowstr);
        result += sum_of_numbers_in_string(rowstr);
    }
    println!("Result: {result}");
}
