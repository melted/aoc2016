fn main() {
    let n = 3018458;
    let mut elves = Vec::with_capacity(n);
    for i in 1..n+1 {
        elves.push(i);
    }
    let mut count = n;
    let mut holes = 0;

    let mut index = 0;
    loop {
        let len = elves.len();
        let mut i = (index + count/2 + holes) % len; 
        elves[i] = 0;
        count -= 1;
        holes += 1;
        let val = elves[index];
        if count == 1 {
            println!("{}", val);
            break;
        }
        index = (index + 1) % len;
        if elves[index] == 0 {
            println!("Compressing {} {}", elves.len(), count);
            elves = elves.into_iter().filter(|e| *e > 0).collect();
            for i in 0..elves.len() {
                if elves[i] == val {
                    index = (i + 1) % elves.len();
                    break;
                }
            }
            holes = 0;
        }
    }
}