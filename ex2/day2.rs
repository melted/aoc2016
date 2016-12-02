use std::io::prelude::*;
use std::fs::File;

const TRANSITIONS_P1 : [[i32 ; 4];10] = [[0,0,0,0],
                                      [1,2,4,1], [2,3,5,1], [3,3,6,2], 
                                      [1,5,7,4], [2,6,8,4], [3,6,9,5], 
                                      [4,8,7,7], [5,9,8,7],[6,9,9,8]];

const TRANSITIONS_P2 : [[i32 ; 4]; 14] = [[0,0,0,0], [1,1,3,1], [2,3,6,2], [1,4,7,2], [4,4,8,3],
                                          [5,6,5,5], [2,7,10,5], [3,8,11,6], [4,9,12,7], [9,9,9,8],
                                          [6,11,10,10], [7,12,13,10],[8,12,12,11], [11,13,13,13]];

fn transition(p2 : bool, n : i32, c : char) -> i32 {
    let trans = if p2 { TRANSITIONS_P2[n as usize] } else { TRANSITIONS_P1[n as usize] };
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

    let mut position1 = 5;
    let mut position2 = 5;

    for l in data.lines() {
        for c in l.chars() {
            position1 = transition(false, position1, c);
            position2 = transition(true, position2, c);
        }
        println!("{} {:x}", position1, position2);
    }
}