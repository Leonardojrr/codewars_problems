//kata: https://www.codewars.com/kata/53e57dada0cb0400ba000688/train/rust
use std::collections::{HashMap, HashSet};

pub fn list_position(word: &str) -> u128 {
    let word_letters = word.chars();

    let mut previous_combinations: u128 = 1;

    for (i, letter) in word_letters.clone().into_iter().enumerate() {
        let mut individual_letters: HashSet<char> = HashSet::new();
        let mut letter_groups: HashMap<char, u8> = HashMap::new();

        for letter in word_letters.clone().into_iter().skip(i) {
            letter_groups
                .entry(letter)
                .and_modify(|c| *c += 1)
                .or_insert(1);

            individual_letters.insert(letter);
        }

        let mut individual_letters: Vec<char> = individual_letters.into_iter().collect();
        individual_letters.sort();

        for sorted_letter in individual_letters {
            if sorted_letter == letter {
                break;
            }

            if sorted_letter < letter {
                letter_groups.entry(sorted_letter).and_modify(|e| *e -= 1);

                previous_combinations += combinations(&letter_groups);
                letter_groups.entry(sorted_letter).and_modify(|e| *e += 1);
            }
        }
    }

    previous_combinations
}

fn combinations(letter_groups: &HashMap<char, u8>) -> u128 {
    let mut denominator = 1;
    let mut numerator: u128 = 0;

    for value in letter_groups.values() {
        numerator += *value as u128;
        denominator *= factorial((*value) as u128);
    }

    factorial(numerator) / denominator
}

fn factorial(n: u128) -> u128 {
    (1..=n).product::<u128>()
}
