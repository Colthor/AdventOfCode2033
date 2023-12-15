use std::{collections::HashSet, fs};

fn main() {
    let filename = std::env::args()
        .nth(1)
        .expect("Pass filename as command line argument");

    println!("Filename: {}", filename);

    let mut total = 0_u32;

    let input = fs::read_to_string(filename).expect("Unable to read file");

    let mut cardcount = vec![1; input.lines().count()];

    for (i, l) in input.lines().enumerate() {
        let mut n = 0_u32;
        let v: Vec<&str> = l.split(&[':', '|']).collect();
        println!("{:#?}", v);
        let mut winners = HashSet::<&str>::new();
        for s in v[1].split(' ') {
            let t = s.trim();
            if !t.is_empty() {
                winners.insert(t);
            }
        }

        for s in v[2].split(' ') {
            if winners.contains(s.trim()) {
                n += 1;
            }
        }
        if n > 0 {
            println!("Card {i} scored {n}");
            for x in (i + 1)..(i + 1 + n as usize) {
                if x < cardcount.len() {
                    println!("Adding {n} of card {x}");
                    cardcount[x] += cardcount[i];
                }
            }
            let score = 2_u32.pow(n - 1);
            total += score;
            println!("Card has {n} wins, scoring {score}");
        }
    }
    println!("Score: {total}");
    //println!("Cardcount: {:#?}", cardcount);
    println!("Cards: {}", cardcount.iter().sum::<u32>());
}
