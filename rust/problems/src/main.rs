// mod even_or_odd_of_list;
// mod recursive_square_finder;
// mod printer_errors;
// mod rbg_to_hex;
// mod last_digit_of_large_number;
// mod path_finder;
// mod trailing_zeros;
// mod largest_index_difference;
// mod simplifying_multilinear_polynomials;
// mod sort_binary_tree_by_levels;
// mod validate_sudoku;
mod number_of_proper_fractions;
use crate::number_of_proper_fractions::proper_fractions;

fn main() {
    print!("{}", proper_fractions(u64::MAX));
}
