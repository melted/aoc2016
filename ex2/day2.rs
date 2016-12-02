use std::io::prelude::*;
use std::fs::File;

const transitions : [[i32 ; 4];10] = [[0,0,0,0],
                                      [1,2,4,1], [2,3,5,1], [3,3,6,2], 
                                      [1,5,7,4], [2,6,8,4], [3,6,9,5], 
                                      [4,8,7,7], [5,9,8,7],[6,9,9,8]];

fn transition(n : i32, c : char) -> i32 {
    let trans = transitions[n as usize];
    match c {
        'U' => trans[0],
        'R' => trans[1],
        'D' => trans[2],
        'L' => trans[3],
        _ => panic!("Bad direction")
    }
}

fn main() {
    let mut inp = File::open("input.txt").expect("Opening input file");
    let mut data = String::new();

    inp.read_to_string(&mut data).expect("Reading input file");

    let mut position = 5;


    for l in data.lines() {
        for c in l.chars() {
            position = transition(position, c);
        }
        print!("{}", position);
    }
    println!("");

}