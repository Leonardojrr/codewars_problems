//Kata: https://www.codewars.com/kata/5511b2f550906349a70004e1/train/rust

pub fn last_digit(str1: &str, str2: &str) -> i32 {
    if str2 == "0" {
        return 1;
    }

    let divisibility_criterion = |module: u32| -> Option<u32> {
        if module == 2 {
            return Some(str2[&str2.len() - 1..str2.len()].parse::<u32>().unwrap());
        }
        if module == 4 {
            if str2.len() > 1 {
                return Some(str2[&str2.len() - 2..str2.len()].parse::<u32>().unwrap());
            } else {
                return Some(str2[&str2.len() - 1..str2.len()].parse::<u32>().unwrap());
            }
        }
        None
    };

    let last_digit_of_base: u8 = str1[str1.len() - 1..str1.len()].parse().unwrap();

    match last_digit_of_base {
        //Only 1 module numbers
        0 => 0,
        1 => 1,
        5 => 5,
        6 => 6,

        // 2 module numbers
        4 | 9 => {
            let module: u32 = 2;
            ((last_digit_of_base.pow(divisibility_criterion(module).unwrap() % module)) % 10) as i32
        }

        // 4 module numbers
        2 | 3 | 7 | 8 => {
            let module: u32 = 4;
            ((last_digit_of_base.pow(divisibility_criterion(module).unwrap() % module)) % 10) as i32
        }
        _ => 0,
    }
}
