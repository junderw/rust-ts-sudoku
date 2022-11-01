import fs from "fs";
import readline from "readline";

type Board = number[][];

const solve = (board: Board): Board => {
  // TODO: probably a better way to copy to ensure recursion doesn't break anything
  let result: Board = JSON.parse(JSON.stringify(board));

  let empty_cell: [number, number] = next_empty_cell(board);

  if (is_solved(board)) {
    return result;
  }

  let row: number = empty_cell[0];
  let column: number = empty_cell[1];

  for (let guess of guesses(board, row, column)) {
    let temp = JSON.parse(JSON.stringify(result))
    temp[row][column] = guess;
    result = solve(temp);

    if (is_solved(result)) {
      return result;
    }
  }

  result = JSON.parse(JSON.stringify(board));
  return result;
}

const next_empty_cell = (board: Board): [number, number] => {
  let row = 0;
  let column = 0;

  while (row < 9) {
    while (column < 9) {
      if (board[row][column] === 0) {
        return [row, column];
      }
      column += 1;
    }
    column = 0;
    row += 1;
  }

  return [-1, -1];
}

const is_solved = (board: Board): boolean => {
  const [row, column] = next_empty_cell(board);
  return row === -1 && column === -1;
}

function guesses(board: Board, row: number, column: number): number[] {
  let result: number[] = [];
  let guess = 1;

  while (guess < 10) {
    if (valid(board, row, column, guess)) {
      result.push(guess);
    }
    guess += 1;
  }

  return result;
}

const valid = (
  board: Board,
  row: number,
  column: number,
  guess: number
): boolean => {
  let x = 0;

  while (x < 9) {
    if (board[row][x] === guess || board[x][column] === guess) {
      return false;
    }
    x += 1;
  }

  let x_index: number = Math.floor(row / 3) * 3;
  let y_index: number = Math.floor(column / 3) * 3;

  x = 0;
  let y = 0;

  while (x < 3) {
    while (y < 3) {
      if (board[x_index + x][y_index + y] === guess) {
        return false;
      }
      y += 1;
    }

    y = 0;
    x += 1;
  }

  return true;
}

(() => {

  const inputStream = fs.createReadStream("./data-files/puzzles.txt");
  var lineReader = readline.createInterface({
    input: inputStream,
    terminal: false,
  });

  const start = performance.now();
  lineReader.on("line", (line) => {
    const puzzle = line.match(/.{1,9}/g)?.reduce((accum: number[][], rowStr) => {
      accum.push(rowStr.split('').map((num) => parseInt(num)))
      return accum;
    }, []);

    if(!puzzle){
      throw new Error(`Couldn't create a puzzle from line: ${line}`)
    }

    console.table(solve(puzzle));
  });

  lineReader.on('close', () => {
    console.log(performance.now() - start)
    console.log('done');
  })
})()
