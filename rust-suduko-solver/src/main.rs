use std::time::Instant;

use itertools::Itertools;

use crate::sudoku_solver::sudoku_solver::solve;
mod read_lines;
mod sudoku_solver;

fn main() {
  let now = Instant::now();
  let file_name = "../data-files/puzzles.txt";

  if let Ok(puzzles) = read_lines::read_lines(file_name) {
    for full_puzzle in puzzles {
      if let Ok(line) = full_puzzle {
        let mut puzzle: [[u32; 9]; 9] = Default::default();

        // Create puzzle
        for (row_index, row) in line.chars().chunks(9).into_iter().enumerate() {
          for (column_index, value) in row.enumerate() {
            match value.to_digit(10) {
              None => panic!("Unknown digit: {value}"),
              Some(x) => puzzle[row_index][column_index] = x,
            };
          }
        }

        println!("{:?}", solve(puzzle));
      }
    }
  }

  // Convert to MS
  let elapsed = now.elapsed().as_secs_f64() * 1000f64;
  println!("Elapsed: {:.2?}", elapsed);
}
