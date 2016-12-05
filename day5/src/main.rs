extern crate md5;

use md5::*;


fn main() {
    let data = "wtnhxymk";
    let mut password = String::new();
    let mut index = 0;

    while password.len() < 8 {
        loop {
            let s = format!("{}{}", data, index);
            let digest = md5::compute(s.as_bytes());
            index = index + 1;
            if digest[0] == 0 && digest[1] == 0 && digest[2] & 0xf0 == 0 {
                println!("----{:?}: {:?}", s, digest);
                password.push_str(&format!("{:x}", digest[2] & 0x0f));
                break;
            }
        }        
    }
    println!("{}", password);
}
