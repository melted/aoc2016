extern crate md5;

use std::collections::BTreeMap;

fn has_triple(hash : &str) -> Option<char> {
    let s = hash.as_bytes();
    for w in s.windows(3) {
        if w[0] == w[1] && w[1] == w[2] {
            return Some(w[1] as char)
        }
    }
    return None
}

fn has_five_of(s : &str, c : &char) -> bool {
    let ascii = (*c as u32) as u8; 
    for w in s.as_bytes().windows(5) {
       if w.iter().all(|d| *d == ascii) {
           return true;
       }
    }
    false
}

fn get_hash_pt2(index : u32) -> String {
    let input = "ahsbgdzn";
    let s = format!("{}{}", input, index);
    let mut hash = format!("{:x}",  md5::compute(s.as_bytes()));
    for _ in 0..2016 {
        hash = format!("{:x}",  md5::compute(hash.as_bytes()));
    }
    hash
}

fn get_hash(index : u32) -> String {
    let input = "ahsbgdzn";
    let s = format!("{}{}", input, index);
    format!("{:x}",  md5::compute(s.as_bytes()))
}

fn get_keys(stretch : bool) -> Vec<u32> {
    let mut triples = BTreeMap::new(); 
    let mut keys = Vec::new(); 
    let mut index = 0;

    while keys.len() < 64 {
        let hash = if stretch { get_hash_pt2(index) } else { get_hash(index) };
        if let Some(c) = has_triple(&hash) {
            triples.insert(index, c);
        }
        for (i, v) in triples.iter_mut() {
            if *i != index && has_five_of(&hash, v) {
                *v = 'g';
                keys.push(*i);
            }
        }
        if index > 999 {
            let _ = triples.remove(&(index - 1000));
        }
        index += 1;
    }
    keys
}

fn main() {
    let keys = get_keys(false);
    let keys2 = get_keys(true);
    println!("{:?}", keys[63]);
    println!("{:?}", keys2[63]);
}

