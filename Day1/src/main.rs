use std::fs;

fn main() {
    println!("Hello, world!");
    let filename = std::env::args().nth(1).expect("Pass filename as command line argument");

    println!("Filename: {}", filename);

    let input = fs::read_to_string(filename).expect("Unable to read file");
    let mut total = 0;
    for l in input.lines() {
        println!("{}", l);
        //let v: Vec<&str> = l.matches(char::is_ascii_digit).collect();
        let v: Vec<&str> = l.matches(char::is_numeric).collect();
        let fst = v.first().unwrap();
        let lst = v.last().unwrap();
        let num_str = format!("{fst}{lst}");
        println!("{}", num_str);
        total += num_str.parse::<i32>().unwrap();
    }
    println!("Total: {}", total);
}
