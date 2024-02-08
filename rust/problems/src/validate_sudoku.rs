//kata:

pub struct Sudoku {
    data: Vec<Vec<u32>>,
}

impl Sudoku {
    pub fn is_valid(&self) -> bool {
        let n = self.data[0].len() as u32;

        if n < 1 {
            return false;
        }

        let valid_result: u32 = (1..=n).sum();

        let mut columns: Vec<Vec<u32>> = vec![];
        for _ in 1..=n {
            columns.push(vec![]);
        }

        //Check Rows
        for row in self.data.iter() {
            let row_sum: u32 = row.iter().sum();

            if row_sum != valid_result {
                return false;
            }

            for (i, value) in row.iter().enumerate() {
                columns[i].push(*value)
            }
        }
        //Check Columns
        for column in columns {
            let column_sum: u32 = column.iter().sum();

            if column_sum != valid_result {
                return false;
            }
        }

        //Check Little Squares
        let squares_n = (n as f32).sqrt() as usize;

        for i in 0..squares_n {
            for j in 0..squares_n {
                let square_sum: u32 = self
                    .data
                    .iter()
                    .skip(i * squares_n)
                    .take(squares_n)
                    .map(|row| row.iter().skip(j * squares_n).take(squares_n))
                    .flatten()
                    .sum();

                if square_sum != valid_result {
                    return false;
                }
            }
        }

        true
    }
}
