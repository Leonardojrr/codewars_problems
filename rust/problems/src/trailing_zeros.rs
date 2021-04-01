// Kata: https://www.codewars.com/kata/52f787eb172a8b4ae1000a34/solutions/rust

pub fn trailing_zeros(n: u128) -> u32 {
    if n < 5 {
        return 0;
    }

    let mut five_multiplers = vec![];

    let mut five_multipler = 5;

    while five_multipler <= n {
        five_multiplers.push(five_multipler);
        five_multipler *= 5;
    }

    let divide = |multiplier: &u128| -> u32 {
        let float_n = n as f64;
        (float_n / (*multiplier as f64)).floor() as u32
    };

    five_multiplers.iter().map(divide).sum()
}
