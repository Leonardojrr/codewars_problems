//kata: https://www.codewars.com/kata/55b7bb74a0256d4467000070/train/rust

pub fn proper_fractions(n: u64) -> u64 {
    if n <= 1 {
        return 0;
    }

    let mut number_of_proper_fractions = n;
    let mut n = n;
    let mut factors = Vec::new();
    let mut divisor = 2;

    while n > 1 {
        while n % divisor == 0 {
            match factors.iter().find(|&x| *x == divisor) {
                Some(_) => {}
                None => factors.push(divisor),
            }

            n /= divisor;
        }
        divisor += 1;
    }

    for prime in factors {
        number_of_proper_fractions = number_of_proper_fractions - number_of_proper_fractions / prime
    }

    number_of_proper_fractions
}
