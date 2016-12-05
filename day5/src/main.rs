extern crate md5;

fn main() {
    let data = "wtnhxymk";
    let mut password = String::new();
    let mut index = 0;
    let mut password2 : [u8; 8] = [255; 8];
    let mut found = 0;

    while found < 8 {
        let s = format!("{}{}", data, index);
        let digest = md5::compute(s.as_bytes());
        index = index + 1;
        if digest[0] == 0 && digest[1] == 0 && digest[2] & 0xf0 == 0 {
            if password.len() < 8 {
                password.push_str(&format!("{:x}", digest[2]));
            }
            let pos = digest[2] as usize;
            if pos < 8 && password2[pos] == 255 {
                found = found + 1;
                password2[pos] = digest[3] >> 4;
            }
        }
    }
    let mut pw2 = String::new();
    for x in password2.iter() {
        pw2.push_str(&format!("{:x}", x));
    }
    println!("{} {}", password, pw2);
}
