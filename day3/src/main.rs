use std::io::prelude::*;
use std::fs::File;

fn load_input() -> String {
    let mut data = String::new();
    let mut fil = File::open("input.txt").expect("Open file");
    fil.read_to_string(&mut data).expect("Reading file");
    data
}

fn main() {
    let data = load_input();
    let triangles : Vec<Vec<i32>> = data.lines()
                                        .map(|s| s.split_whitespace()
                                                  .map(|n| i32::from_str_radix(n, 10).expect("Conversion"))
                                                  .collect())
                                        .collect();
    let valid = triangles.iter().map(|v| {
        let total : i32 = v.iter().sum();
        v.iter().map(|n| n + n - total).all(|x| x < 0)
    }).filter(|&b| b).count();

    println!("{:?}/{}", valid, triangles.len());
}
