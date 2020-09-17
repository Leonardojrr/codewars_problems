//Kata: https://www.codewars.com/kata/5511b2f550906349a70004e1/train/rust

pub fn last_digit(str1: &str, str2: &str) -> i32 {
    if str2 == "0" {
        return 1;
    }

    let divisibility_criterion = |module: u8| -> Option<u8> {
        if module == 2 {
            return Some(str2[&str2.len() - 1..str2.len()].parse::<u8>().unwrap());
        }
        if module == 4 {
            if str2.len() > 1 {
                return Some(str2[&str2.len() - 2..str2.len()].parse::<u8>().unwrap());
            } else {
                return Some(str2[&str2.len() - 1..str2.len()].parse::<u8>().unwrap());
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
        4 => match divisibility_criterion(2).unwrap() % 2 {
            0 => 6,
            1 => 4,
            _ => 0,
        },
        9 => match divisibility_criterion(2).unwrap() % 2 {
            0 => 1,
            1 => 9,
            _ => 0,
        },

        // 4 module numbers
        2 => match divisibility_criterion(4).unwrap() % 4 {
            0 => 6,
            1 => 2,
            2 => 4,
            3 => 8,
            _ => 0,
        },
        3 => match divisibility_criterion(4).unwrap() % 4 {
            0 => 1,
            1 => 3,
            2 => 9,
            3 => 7,
            _ => 0,
        },
        7 => match divisibility_criterion(4).unwrap() % 4 {
            0 => 1,
            1 => 7,
            2 => 9,
            3 => 3,
            _ => 0,
        },
        8 => match divisibility_criterion(4).unwrap() % 4 {
            0 => 6,
            1 => 8,
            2 => 4,
            3 => 2,
            _ => 0,
        },
        _ => 0,
    }
}
