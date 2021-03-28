//Kata: https://www.codewars.com/kata/5765870e190b1472ec0022a2/train/rust

struct Tile {
    visited: bool,
    value: char,
}

impl Tile {
    fn new(value: char) -> Self {
        Self {
            visited: false,
            value,
        }
    }
}

struct Board {
    tiles: Vec<Vec<Tile>>,
    x_len: i32,
    y_len: i32,
}

impl Board {
    fn new(maze: &str) -> Self {
        let mut tiles = vec![];

        for row in maze.split("\n") {
            let mut columns = vec![];

            for value in row.chars() {
                columns.push(Tile::new(value));
            }
            tiles.push(columns);
        }

        let x_len = (tiles[0].len() - 1) as i32;
        let y_len = (tiles.len() - 1) as i32;

        Self {
            tiles,
            x_len,
            y_len,
        }
    }

    fn tile(&self, x: i32, y: i32) -> &Tile {
        &self.tiles[y as usize][x as usize]
    }

    fn mut_tile(&mut self, x: i32, y: i32) -> &mut Tile {
        &mut self.tiles[y as usize][x as usize]
    }

    fn can_vitit(&self, x: i32, y: i32) -> bool {
        if x <= self.x_len && y <= self.y_len && x >= 0 && y >= 0 {
            if !self.tile(x, y).visited && self.tile(x, y).value != 'W' {
                return true;
            }
        }
        false
    }

    fn is_exit(&self, x: i32, y: i32) -> bool {
        if x == self.x_len && y == self.y_len {
            return true;
        }
        false
    }

    fn visit(&mut self, x: i32, y: i32) -> bool {
        if self.is_exit(x, y) {
            return true;
        }

        self.mut_tile(x, y).visited = true;
        false
    }
}

pub fn path_finder(maze: &str) -> bool {
    fn find_exit(board: &mut Board, x: i32, y: i32) -> bool {
        if board.visit(x, y) {
            return true;
        }

        if board.can_vitit(x, y + 1) {
            if find_exit(board, x, y + 1) {
                return true;
            }
        }

        if board.can_vitit(x + 1, y) {
            if find_exit(board, x + 1, y) {
                return true;
            }
        }

        if board.can_vitit(x, y - 1) {
            if find_exit(board, x, y - 1) {
                return true;
            }
        }

        if board.can_vitit(x - 1, y) {
            if find_exit(board, x - 1, y) {
                return true;
            }
        }

        false
    }

    let mut board = Board::new(maze);

    find_exit(&mut board, 0, 0)
}
