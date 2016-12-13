use std::collections::HashSet;

type Point = (usize, usize);
const WIDTH : usize = 100;
const HEIGHT : usize = 100;

fn p_to_i(pt : Point) -> usize {
    pt.1*WIDTH + pt.0
}

fn is_wall((x, y) : Point) -> bool {
    let input = 1364;
    let sum = x*x + y*y + 2*x*y + 3*x + y + input;
    usize::count_ones(sum) % 2 == 1
}

fn neighbours(pt : Point) -> Vec<Point> {
    let mut out = Vec::new();
    if pt.0 > 0 {
        out.push((pt.0 - 1, pt.1));
    }
    if pt.1 > 0 {
        out.push((pt.0, pt.1 - 1));    
    }
    if pt.0 < WIDTH - 1 {
        out.push((pt.0 + 1, pt.1)); 
    }
    if pt.1 < HEIGHT - 1 {
        out.push((pt.0, pt.1 + 1));    
    }
    out
}

fn find_pos_and_count(start : Point, target : Point) -> (u32, u32) {
    let mut round = 0;
    let mut dist = Vec::new();
    dist.resize(WIDTH*HEIGHT, 99999);
    let mut to_explore = HashSet::new();
    to_explore.insert(start);

    loop {
        let mut next_round = HashSet::new();
        for c in to_explore {
            dist[p_to_i(c)] = round;
            if c == target {
                let count = dist.iter().filter(|x| **x < 51).count() as u32;
                return (round, count);
            }
            for n in neighbours(c) {
                if dist[p_to_i(n)] == 99999 {
                    if !is_wall(n) {
                        next_round.insert(n);
                    } else {
                        dist[p_to_i(n)] = 1000000;
                    }
                }
            }
        }
        round += 1;
        to_explore = next_round;
    }
}

fn main() {
    let (time, locs) = find_pos_and_count((1,1), (31,39));
    println!("{:?} {}", time, locs);      
}
