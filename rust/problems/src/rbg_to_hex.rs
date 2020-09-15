// Kata: https://www.codewars.com/kata/513e08acc600c94f01000001/train/rust

pub fn rgb(r: i32, g: i32, b: i32) -> String {
    let mut array = [r, g, b];

    for n in array.iter_mut() {
        if *n < 0 {
            *n = 0;
        }
        if *n > 255 {
            *n = 255;
        }
    }

    format!("{:02X}{:02X}{:02X}", array[0], array[1], array[2])
}
