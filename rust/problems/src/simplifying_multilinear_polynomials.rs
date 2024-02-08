// Kata: https://www.codewars.com/kata/55f89832ac9a66518f000118/train/rust

use std::collections::HashMap;

pub fn simplify(polynomial: &str) -> String {
    let chars: Vec<char> = polynomial.chars().collect();
    let mut polynoms: HashMap<String, i32> = HashMap::new();

    let mut is_negative = false;
    let mut coefficient_string = String::new();
    let mut polynom: Option<String> = None;

    for (i, char) in chars.iter().enumerate() {
        if *char == '+' {
            is_negative = false;
            continue;
        }

        if *char == '-' {
            is_negative = true;
            continue;
        }

        if char.is_digit(10) {
            coefficient_string.push(*char);
            continue;
        }

        if char.is_alphabetic() {
            match &mut polynom {
                Some(polynom_string) => polynom_string.push(*char),

                None => polynom = Some(String::from(*char)),
            }

            match chars.get(i + 1) {
                Some(char) => {
                    if *char == '+' || *char == '-' {
                        match polynom {
                            Some(polynom_str) => {
                                let mut polynom_chars: Vec<char> = polynom_str.chars().collect();
                                polynom_chars.sort();

                                let mut coefficient: i32 = coefficient_string.parse().unwrap_or(1);
                                let lexicografic_polynom: String = polynom_chars.iter().collect();

                                if is_negative {
                                    coefficient = coefficient * -1
                                }

                                match polynoms.get_mut(&lexicografic_polynom) {
                                    Some(coefficient_sum) => {
                                        *coefficient_sum = *coefficient_sum + coefficient
                                    }

                                    None => {
                                        polynoms.insert(lexicografic_polynom, coefficient);
                                    }
                                }

                                coefficient_string.clear();
                                polynom = None;
                            }

                            None => {}
                        }
                    }
                }

                None => match polynom {
                    Some(polynom_str) => {
                        let mut polynom_chars: Vec<char> = polynom_str.chars().collect();
                        polynom_chars.sort();

                        let mut coefficient: i32 = coefficient_string.parse().unwrap_or(1);
                        let lexicografic_polynom: String = polynom_chars.iter().collect();

                        if is_negative {
                            coefficient = coefficient * -1
                        }

                        match polynoms.get_mut(&lexicografic_polynom) {
                            Some(coefficient_sum) => {
                                *coefficient_sum = *coefficient_sum + coefficient
                            }

                            None => {
                                polynoms.insert(lexicografic_polynom, coefficient);
                            }
                        }

                        coefficient_string.clear();
                        polynom = None;
                    }

                    None => {}
                },
            }
        }
    }

    let mut polynoms: Vec<(String, i32)> = polynoms
        .into_iter()
        .filter(|&(_, value)| value != 0)
        .collect();

    polynoms.sort_by_key(|(s, _)| (s.as_str().len(), s.clone()));

    let mut output_string = String::new();

    for (i, (polynom, coefficient)) in polynoms.into_iter().enumerate() {
        let mut format = String::new();

        if coefficient == -1 {
            format = format!("-{polynom}");
        } else {
            if i == 0 {
                if coefficient == 1 {
                    format = polynom
                } else {
                    format = format!("{coefficient}{polynom}");
                }
            } else {
                if coefficient == 1 {
                    format = format!("+{polynom}");
                } else {
                    format = format!("{coefficient:+}{polynom}")
                }
            }
        }

        output_string.push_str(&format);
    }

    output_string
}
