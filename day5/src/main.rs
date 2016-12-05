extern crate md5;

fn main() {
    let data = "wtnhxymk";
    let mut password = String::new();
    let mut index = 0;

    while password.len() < 8 {
        let s = format!("{}{}", data, index);
        let digest = md5::compute(s.as_bytes());
        index = index + 1;
        if digest[0] == 0 && digest[1] == 0 && digest[2] & 0xf0 == 0 {
            password.push_str(&format!("{:x}", digest[2] & 0x0f));
        }
    }

    println!("{}", password);

    let mut password2 : [u8; 8]= [255; 8];
    let mut found = 0;
    index = 0;

    while found < 8 {
        let s = format!("{}{}", data, index);
        let digest = md5::compute(s.as_bytes());
        index = index + 1;
        if digest[0] == 0 && digest[1] == 0 && digest[2] & 0xf0 == 0 {
            let pos = digest[2] & 0x0f;
            let val = (digest[3] & 0xf0) >> 4;
            if pos < 8 && password2[pos as usize] == 255 {
                found = found + 1;
                password2[pos as usize] = val;
            }
        }
    }

    println!("{:?}", password2);
}
