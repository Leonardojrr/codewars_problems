// mod even_or_odd_of_list;
// mod recursive_square_finder;
// mod printer_errors;
// mod rbg_to_hex;
// mod last_digit_of_large_number;
mod path_finder;

use path_finder::path_finder;

fn main() {
    path_finder(
        "\
            ......\n\
            ......\n\
            ......\n\
            ......\n\
            ......\n\
            ......\
            ",
    );
}
