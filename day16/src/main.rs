
fn fill_op(a : &str) -> String {
    let b : String = a.chars().rev().map(|c| match c { '0' => '1', '1' => '0', _ => panic!("!!!") }).collect();
    let out = a.to_string() + "0" + b.as_str();
    out
}

fn filler(s : &str, size : usize) -> String {
    let mut out = fill_op(s);
    while out.len() < size {
        out = fill_op(&out);
    }
    out.chars().take(size).collect()
}

fn checksum(s : &str) -> String {
    let mut out = s.to_string();
    loop {
        let mut shrunk = String::new();
        for c in out.as_bytes().chunks(2) {
            let v = if c[0] == c[1] { '1' } else { '0' };
            shrunk.push(v);   
        }
        out = shrunk;
        if out.len() % 2 == 1 {
            return out;    
        }
    }
}

fn main() {
    let input = "00101000101111010";
    let part1 = filler(&input, 272);
    let part2 = filler(&input, 35651584);
    let check1 = checksum(&part1);
    let check2 = checksum(&part2);
    println!("{} {}", check1, check2);
}
