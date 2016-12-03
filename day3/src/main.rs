use std::io::prelude::*;
use std::fs::File;

fn load_input() -> String {
    let mut data = String::new();
    let mut fil = File::open("input.txt").expect("Open file");
    fil.read_to_string(&mut data).expect("Reading file");
    data
}

fn valid_count(v : &Vec<Vec<i32>>) -> usize {
    v.iter().filter(|u| {
        let total : i32 = u.iter().sum();
        u.iter().map(|n| n + n - total).all(|x| x < 0)
    }).count()
}

fn get_row(s : &str) -> Vec<i32> {
    s.split_whitespace().map(|n| i32::from_str_radix(n, 10).expect("Conversion")).collect()
}

fn part2_triangles(s : &str) -> Vec<Vec<i32>> {
    let mut values = Vec::new();
    let n = s.lines().count();
    values.resize(n * 3, 0);

    for (i, l) in s.lines().enumerate() {
        let v = get_row(l);
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

fn p2_triangles(t : &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut out = Vec::new();
    let n = t.len();
    for i in 0..n {
        let f = |z| *t.get((i*3)%n+z).unwrap().get((i*3+z)/n).unwrap();
        out.push([f(0),f(1),f(2)].to_vec());
    }
    out
}

fn main() {
    let data = load_input();
    let triangles = data.lines().map(|s| get_row(s)).collect();
    let triangles2 = part2_triangles(&data);
    let t2 = p2_triangles(&triangles);

    println!("{} {} {}", valid_count(&triangles), valid_count(&triangles2), valid_count(&t2));
}
