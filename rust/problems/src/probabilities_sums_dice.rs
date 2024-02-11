//kata: https://www.codewars.com/kata/56f78a42f749ba513b00037f/train/rust

pub fn count_combinations(sum: i32, dice_amount: i32) -> i32 {
    if dice_amount == 0 {
        return if sum == 0 { 1 } else { 0 };
    }

    let mut count = 0;

    for face in 1..=6 {
        if face <= sum {
            count += count_combinations(sum - face, dice_amount - 1);
        }
    }

    count
}

pub fn rolldice_sum_prob(sum: i32, dice_amount: i32) -> f64 {
    count_combinations(sum, dice_amount) as f64 / 6f64.powf(dice_amount as f64)
}
