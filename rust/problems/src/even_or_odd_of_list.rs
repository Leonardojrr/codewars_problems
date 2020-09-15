// You are given an array (which will have a length of at least 3, but could be very large) containing integers.
// The array is either entirely comprised of odd integers or entirely comprised of even integers except for a single integer N.
//  Write a method that takes the array as an argument and returns this "outlier" N.

// Examples
// [2, 4, 0, 100, 4, 11, 2602, 36]
// Should return: 11 (the only odd number)

// [160, 3, 1719, 19, 11, 13, -21]
// Should return: 160 (the only even number)

//Kata: https://www.codewars.com/kata/5526fc09a1bbd946250002dc/train/rust

pub fn find_outlier(values: &[i32]) -> i32 {
    let find_odd = || {
        for num in values {
            if num % 2 == 0 {
                return *num;
            }
        }
        0
    };

    let find_even = || {
        for num in values {
            if !(num % 2 == 0) {
                return *num;
            }
        }
        0
    };

    if &values[0] % 2 == 0 && values[1] % 2 == 0
        || values[0] % 2 == 0 && values[2] % 2 == 0
        || values[1] % 2 == 0 && values[2] % 2 == 0
    {
        find_even()
    } else {
        find_odd()
    }
}
