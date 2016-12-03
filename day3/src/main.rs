use std::io::prelude::*;
use std::fs::File;

fn load_input() -> String {
    let mut data = String::new();
    let mut fil = File::open("input.txt").expect("Open file");
    fil.read_to_string(&mut data).expect("Reading file");
    data
}

fn valid_count(v : &Vec<Vec<i32>>) -> usize {
    v.iter().map(|v| {
        let total : i32 = v.iter().sum();
        v.iter().map(|n| n + n - total).all(|x| x < 0)
    }).filter(|&b| b).count()
}

fn part2_triangles(s : &str) -> Vec<Vec<i32>> {
    let mut values : Vec<i32> = Vec::new();
    let n = s.lines().count();
    values.resize(n * 3, 0);

    for (i, l) in s.lines().enumerate() {
        let v : Vec<i32> = l.split_whitespace()
                            .map(|n| i32::from_str_radix(n, 10).expect("Conversion"))
                            .collect();
        for p in 0..3 {
            *values.get_mut(i + p*n).unwrap() = *v.get(p).unwrap();
        }
    }
    let mut out = Vec::new();

    for c in values.chunks(3) {
        out.push(c.to_vec());
    }
    out
}

fn main() {
    let data = load_input();
    let triangles : Vec<Vec<i32>> = data.lines()
                                        .map(|s| s.split_whitespace()
                                                  .map(|n| i32::from_str_radix(n, 10).expect("Conversion"))
                                                  .collect())
                                        .collect();
    let triangles2 = part2_triangles(&data);

    println!("{:?}/{}", valid_count(&triangles), triangles.len());
    println!("{:?}/{}", valid_count(&triangles2), triangles2.len());
}
