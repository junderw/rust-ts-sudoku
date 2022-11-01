// https://blog.cloudboost.io/sudoku-solver-rust-recursive-implementation-backtracking-technique-fecf87d0477

pub mod sudoku_solver {
    pub fn valid(board: [[u32; 9]; 9], row: usize, column: usize, guess: u32) -> bool {
        // Check row and column for the guess
        for x in 0..9 {
            if board[row][x] == guess || board[x][column] == guess {
                return false;
            }
        }

        let x_index = row / 3 * 3;
        let y_index = column / 3 * 3;

        // Check the containing square for the guess
        for x in 0..3 {
            for y in 0..3 {
                if board[x_index + x][y_index + y] == guess {
                    return false;
                }
            }
        }

        true
    }

    // Consider puzzle solved when an empty cell cannot be found
    fn is_solved(board: [[u32; 9]; 9]) -> bool {
        next_empty_cell(board) == [10, 10]
    }

    // Find the next empty cell on the board
    fn next_empty_cell(board: [[u32; 9]; 9]) -> [usize; 2] {
        for row in 0..9 {
            for column in 0..9 {
                if board[row][column] == 0 {
                    return [row, column];
                }
            }
        }

        [10, 10]
    }

    // Find all valid options for a position in the puzzle
    fn get_options(board: [[u32; 9]; 9], row: usize, column: usize) -> Vec<u32> {
        let mut result = vec![];

        for option in 1..10 {
            if valid(board, row, column, option) {
                result.push(option);
            }
        }

        result
    }

    pub fn solve(board: [[u32; 9]; 9]) -> [[u32; 9]; 9] {
        let mut result = board;

        let empty_cell: [usize; 2] = next_empty_cell(board);

        if is_solved(board) {
            return result;
        }

        let row: usize = empty_cell[0];
        let column: usize = empty_cell[1];

        for option in get_options(board, row, column) {
            result[row][column] = option;
            result = solve(result);
            if is_solved(result) {
                return result;
            }
        }
        result = board;
        return result;
    }
}
