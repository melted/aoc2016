use std::io::prelude::*;
use std::fs::File;
use std::collections::HashSet;


fn str_to_turn(s:&str) -> i32 {
    match s {
        "R" => 1,
        "L" => -1,
        _ => panic!("Illegal turn")
    }
}

fn turn(dir : i32, turn : i32) -> i32 {
    let v = (dir + turn) % 4;
    if v < 0 { v + 4 } else { v } 
}

fn update((x, y) : (i32, i32), dir : i32, steps : i32) -> (i32, i32) {
    match dir {
        0 => (x, y+steps),
        1 => (x+steps, y),
        2 => (x, y-steps),
        3 => (x-steps, y),
        _ => panic!("bad direction")
    }
}

fn main() {
    let mut inp = File::open("input.txt").expect("Failed to open input");
    let mut data = String::new();

    inp.read_to_string(&mut data).expect("Nothing to read");
    let v:Vec<(i32, i32)> = data.split(',')
                                 .map(|s| s.trim().split_at(1))
                                 .map(|(t, n)| (str_to_turn(t), i32::from_str_radix(n, 10).expect("not an number")))
                                 .collect();
    let initial_pos = (0, (0, 0));

    let (_, (fx, fy)) = v.iter().fold(initial_pos, 
                            |(dir, pos), &(t, s)| {
                                let new_dir = turn(dir, t);
                                (new_dir, update(pos, new_dir, s))
                            });

    println!("Part One: {:?}", fx.abs() + fy.abs());
    
    // Part Two
    let mut positions : HashSet<(i32, i32)> = HashSet::new();
    let mut current = initial_pos;

    'nested: for (t, s) in v {
        let (dir, pos) = current;
        let new_dir = turn(dir, t);
        let mut new_pos = pos;
        for _ in 0..s {
            new_pos = update(new_pos, new_dir, 1);
            current = (new_dir, new_pos);
            if positions.contains(&new_pos) {
                break 'nested;    
            }
            positions.insert(new_pos);
        }
    } 

    println!("Part Two: {:?}", (current.1).0.abs() + (current.1).1.abs());
}