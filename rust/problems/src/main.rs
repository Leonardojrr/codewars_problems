// mod even_or_odd_of_list;
// mod recursive_square_finder;
// mod printer_errors;
// mod rbg_to_hex;
mod last_digit_of_large_number;

fn main() {
    // fn are_congruent(int1: u128, int2: u128, module: u128) {
    //     if int1 % module == int2 % module {
    //         println!("{} ≡ {} (mod {})", int1, int2, module);
    //     } else {
    //         println!("Not congruent");
    //     }
    // }

    // let expo = "6881961522155299727373717455716565748342736220751795265";
    // let len = expo.len();
    // println!("{}", len);

    // let decimas = 1;
    // let numero = 1;

    // match decimas % 2 {
    //     0 => match numero % 4 {
    //         1 => {
    //             println!("2");
    //         }
    //         2 => {
    //             println!("4");
    //         }
    //         3 => {
    //             println!("8");
    //         }
    //         0 => {
    //             println!("6");
    //         }
    //         _ => {
    //             println!("xd");
    //         }
    //     },
    //     1 => match numero % 4 {
    //         1 => {
    //             println!("4");
    //         }
    //         2 => {
    //             println!("8");
    //         }
    //         3 => {
    //             println!("6");
    //         }
    //         0 => {
    //             println!("2");
    //         }
    //         _ => {
    //             println!("xd");
    //         }
    //     },
    //     _ => {
    //         println!("xd");
    //     }
    // }

    // match 8 as u128 % 4 {
    //     1 => println!("es 3"),
    //     2 => println!("es 9"),
    //     3 => println!("es 7"),
    //     0 => println!("es 1"),
    //     _ => println!("puta"),
    // }

    // match expo % 4 {
    //     0 => result = 2,
    //     1 => result = 4,
    //     2 => result = 8,
    //     3 => result = 6,
    //     _ => result = 0,
    // }

    // println!(
    //     "the number is :{} answer:{} opt:{}",
    //     num.pow(expo),
    //     result,
    //     expo % 4
    // );

    println!("{}", last_digit_of_large_number::last_digit("4", "1"));

    // let mut vec: Vec<u128> = Vec::new();
    // let decimas = 20;
    // let module = 3;

    // for decima in 0..decimas + 1 {
    //     vec.push((10 as u128).pow(decima));
    // }

    // for decima in vec {
    //     println!("{} {}", decima, decima % module);
    // }
}
