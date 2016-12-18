use std::io::prelude::*;
use std::fs::File;

type Row = Vec<u32>;

fn load_input() -> String {
    let mut data = String::new();
    let mut fil = File::open("input.txt").expect("Open file");
    fil.read_to_string(&mut data).expect("Reading file");
    data
}

fn parse_input(s : &str) -> Row {
    let mut out = Vec::new();
    for c in s.chars() {
        match c {
            '.' => out.push(1),
            '^' => out.push(0),
            '\n' => {},
            x => panic!("Invalid input {}", x)
        }
    }
    out
}

fn tile_value(l : u32, c : u32, r : u32) -> u32 {
    if l == c && r != c || r == c &&  l != c {
        0
    } else {
        1
    }
} 

fn make_next_row(r : &Row) -> Row {
    let mut out = Vec::with_capacity(r.len());
    out.push(tile_value(1, r[0], r[1]));
    for w in r.as_slice().windows(3) {
        out.push(tile_value(w[0], w[1], w[2]));
    }
    out.push(tile_value(r[r.len()-2], r[r.len()-1], 1));
    out
}

fn main() {
    let input = load_input();
    let mut row = parse_input(&input);
    let mut count : u32 = 0;
    let mut part1 = 0;

    for i in 0..400000 {
        count += row.iter().sum();
        if i == 39 {
            part1 = count;
        }
        row = make_next_row(&row); 
    }

    println!("{} {}", part1, count);
}
