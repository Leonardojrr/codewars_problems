// Kata: https://www.codewars.com/kata/56541980fa08ab47a0000040/train/rust

pub fn printer_error(s: &str) -> String {
    let mut count: u8 = 0;
    for c in s.chars() {
        if c > 'm' {
            count += 1;
        }
    }

    format!("{}/{}", count, s.len())
}
