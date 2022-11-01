// https://blog.cloudboost.io/sudoku-solver-rust-recursive-implementation-backtracking-technique-fecf87d0477

pub type SudokuBoard = [[u8; 9]; 9];

#[inline]
pub fn valid(board: &SudokuBoard, row: usize, column: usize, guess: u8) -> bool {
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
#[inline]
fn is_solved(board: &SudokuBoard) -> bool {
    next_empty_cell(board) == [10, 10]
}

// Find the next empty cell on the board
#[inline]
fn next_empty_cell(board: &SudokuBoard) -> [usize; 2] {
    for (row, row_array) in board.iter().enumerate() {
        for (column, &item) in row_array.iter().enumerate() {
            if item == 0 {
                return [row, column];
            }
        }
    }

    [10, 10]
}

// Find all valid options for a position in the puzzle
#[inline]
fn get_options(board: &SudokuBoard, row: usize, column: usize) -> [u8; 9] {
    let mut result = [0_u8; 9];
    let mut index = 0;

    for option in 1..10 {
        if valid(board, row, column, option) {
            result[index] = option;
            index += 1;
        }
    }

    result
}

#[inline]
pub fn solve(mut board: SudokuBoard) -> SudokuBoard {
    let [row, column] = next_empty_cell(&board);

    if [row, column] == [10, 10] {
        return board;
    }

    for option in get_options(&board, row, column)
        .into_iter()
        .take_while(|&x| x != 0)
    {
        board[row][column] = option;
        board = solve(board);
        if is_solved(&board) {
            return board;
        }
    }
    board[row][column] = 0;
    board
}
