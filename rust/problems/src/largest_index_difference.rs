// Kata: https://www.codewars.com/kata/52503c77e5b972f21600000e/train/rust

pub fn largest_index_difference(data: &[i32]) -> usize {
    let mut largest_diff = 0;

    for (i, i_number) in data.iter().enumerate() {
        for j in (i + 1)..data.len() {
            if data[j] >= *i_number {
                let diff = j - i;

                if diff > largest_diff {
                    largest_diff = diff
                }
            }
        }
    }

    return largest_diff;
}
