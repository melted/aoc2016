fn get_elves(n : usize) -> Vec<usize> {
    let mut out = Vec::with_capacity(n);
    for i in 1..n + 1 {
        out.push(i)
    }
    out
}

fn play_game(n : usize, part1 : bool) -> usize {
    let mut elves = get_elves(n);
    let mut count = n;

    let mut index = 0;
    loop {
        let mut target = if part1 { 1 } else { count/2 };
        let mut i = index;
        while target > 0 {
            i = (i + 1) % elves.len();
            if elves[i] > 0 {
                target -= 1;
            }
        }
        elves[i] = 0;
        count -= 1;
        if count == 1 {
            return elves[index];
        }
        loop {
            index += 1;
            if index == elves.len() {
                println!("Compressing {} {}", elves.len(), count);
                elves = elves.into_iter().filter(|e| *e > 0).collect();
                index = 0;
            }
            if elves[index] > 0 {
                break;
            }
        }
    }
}

fn main() {
    let input = 3018458;
    println!("{}", play_game(input, true), );
    println!("{}", play_game(input, false));
}
