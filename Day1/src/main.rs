use std::fs;

const NUMSTRS: &[(&str, &str, usize, &str)] = &[
        ("one", "1  ", 1, "1"),
        ("two", "2  ", 2, "2"),
        ("three", "3    ", 3, "3"),
        ("four", "4   ", 4, "4"),
        ("five", "5   ", 5, "5"),
        ("six", "6  ", 6, "6"),
        ("seven", "7    ", 7, "7"),
        ("eight", "8    ", 8, "8"),
        ("nine", "9   ", 9, "9"),
    ];

fn first(in_str: &str) -> i32 {
    let mut v = vec![usize::MAX; 10];
    for n in NUMSTRS {
        let r_str = in_str.replace(n.0, n.1);
        v[n.2] = r_str.find(n.3).unwrap_or(usize::MAX);
    }
    //println!("{:?}", v);
    let first_index = *v.iter().min().unwrap();
    i32::try_from(v.iter().position(|x|  *x == first_index ).unwrap()).ok().unwrap()
}
fn last(in_str: &str) -> i32 {
    let mut v = vec![usize::MIN; 10];
    for n in NUMSTRS {
        let r_str = in_str.replace(n.0, n.1);
        let found = r_str.rfind(n.3);
        match found {
            None => (),
            Some(x) => v[n.2] = 1 + x, // + 1 because usize::MIN is zero, the same as the first character index
        }
    }
    //println!("{:?}", v);
    let first_index = *v.iter().max().unwrap();
    i32::try_from(v.iter().position(|x|  *x == first_index ).unwrap()).ok().unwrap()
}

fn main() {
    println!("Hello, world!");
    let filename = std::env::args().nth(1).expect("Pass filename as command line argument");

    println!("Filename: {}", filename);

    let input = fs::read_to_string(filename).expect("Unable to read file");
    let mut total = 0;
    for l in input.lines() {
        println!("{}", l);

        let fst = first(l);
        let lst = last(l);
        let s_num = fst * 10 + lst;

        println!("first: {}, last: {}, number: {}", fst, lst, s_num);
        total += s_num;
    }
    println!("Total: {}", total);
}
