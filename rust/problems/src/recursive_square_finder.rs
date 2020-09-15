// Kata: https://www.codewars.com/kata/55466989aeecab5aac00003e/train/rust

pub fn sq_in_rect(lng: i32, wdth: i32) -> Option<Vec<i32>> {
    if lng == wdth {
        return None;
    }

    fn recursive(lng: i32, wdth: i32) -> Option<Vec<i32>> {
        let mut vec: Vec<i32> = Vec::new();
        if lng == wdth {
            vec.push(lng);
            return Some(vec);
        }
        if lng > wdth {
            vec.push(wdth);
            vec.append(&mut recursive(lng - wdth, wdth).unwrap());
        } else {
            vec.push(lng);
            vec.append(&mut recursive(lng, wdth - lng).unwrap());
        }
        Some(vec)
    }
    Some(recursive(lng, wdth).unwrap())
}
