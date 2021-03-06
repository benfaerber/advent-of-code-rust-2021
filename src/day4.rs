#[path = "./advent.rs"]
mod advent;

fn get_moves(raw_moves: &str) -> Vec<i32> {
  let moves: Vec<i32> = raw_moves
  .trim()
  .split(",")
  .map(|v| v.trim().parse::<i32>().unwrap())
  .collect();

  moves
}

fn get_board(raw_board: &str) -> Vec<Vec<i32>> {
  let rows: Vec<&str> = raw_board.split("\n").collect();
  let grid: Vec<Vec<i32>> = rows
  .iter()
  .map(|row| {
    let cols: Vec<i32> = row
    .split(" ")
    .filter(|v| !v.is_empty())
    .map(|v| v.trim().parse::<i32>().unwrap())
    .collect();

    cols
  })
  .collect();

  grid
}

fn flatten_board(board: &Vec<Vec<i32>>) -> Vec<i32> {
  let size: usize = board.len() ** &2;
  let all_nums: Vec<i32> = board
  .iter()
  .fold(vec![], |mut acc, curr| {
    acc.extend(curr);
    acc
  });

  all_nums
}

fn get_chunks() -> Vec<String> {
  let raw_data = advent::read_as_string("day4.txt");
  let chunks: Vec<String> = raw_data
  .trim()
  .split("\n\n") // Boards have a blank line between them
  .map(|v| v.to_string())
  .collect();

  chunks
}

fn is_win(board: &Vec<Vec<i32>>, moves: &Vec<i32>) -> bool {
  let range = 0..board.len();
  let win: bool = range
  .fold(false, |has_won, index| {

    let is_hori: bool = board[index]
    .iter()
    .fold(true, |is_all, number| if moves.contains(number) {is_all} else {false});


    let is_veri: bool = board
    .iter()
    .fold(true, |is_all, row| {
      let col = &row[index];
      if moves.contains(col) {is_all} else {false}
    });
    //board[index]
    if is_veri || is_hori {true} else {has_won}
  });

  win
}

fn get_unchecked(mut flat_board: Vec<i32>, move_set: Vec<i32>) -> Vec<i32> {
  let unchecked: Vec<i32> = flat_board
  .drain(..)
  .filter(|v| !move_set.contains(v))
  .collect();

  unchecked
}

pub fn part1() {
  let chunks: Vec<String> = get_chunks();

  let raw_moves = &chunks[0];
  let raw_boards = &chunks[1..];

  let moves: Vec<i32> = get_moves(raw_moves);
  let boards: Vec<Vec<Vec<i32>>> = raw_boards
  .iter()
  .map(|raw_board| get_board(raw_board))
  .collect();


  let inner = vec![vec![-1]];
  let null_board = vec![&inner];

  let move_range = 0..moves.len();

  // I must cheat and use a mutible ref :(
  let mut move_index = 0;

  let won_board = move_range
  .fold(null_board, |acc, index| {
    let current_moves: Vec<i32> = moves[0..=index].to_vec();
    //println!("{:?}", current_moves);
    let won_board: Vec<&Vec<Vec<i32>>> = boards
    .iter()
    .filter(|b| is_win(&b, &current_moves))
    .collect();

    let has_found = won_board.len() != 0;
    let is_first_found = acc[0][0][0] == -1;
    if has_found && is_first_found {
      move_index = index;
      won_board
    } else { acc }
  });

  let win_board_nums = flatten_board(&won_board[0]);
  let move_set: Vec<i32> = moves[0..=move_index].to_vec();

  let unchecked_values = get_unchecked(win_board_nums, move_set);

  let uncheck_sum: i32 = unchecked_values.iter().sum();
  let last_called_val = moves[move_index];
  let multi = uncheck_sum * last_called_val;
  println!("{}", multi);
}

pub fn part2() {
  let chunks: Vec<String> = get_chunks();

  let raw_moves = &chunks[0];
  let raw_boards = &chunks[1..];

  let moves: Vec<i32> = get_moves(raw_moves);
  let boards: Vec<Vec<Vec<i32>>> = raw_boards
  .iter()
  .map(|raw_board| get_board(raw_board))
  .collect();


  let inner = vec![vec![-1]];
  let null_board = vec![&inner];

  let move_range = 0..moves.len();

  // I must cheat and use a mutible ref :(
  let mut win_index: usize = 0;

  let lost_board = move_range
  .fold(null_board, |acc, index| {
    let current_moves: Vec<i32> = moves[0..=index].to_vec();
    //println!("{:?}", current_moves);
    let lost_board: Vec<&Vec<Vec<i32>>> = boards
    .iter()
    .filter(|b| !is_win(&b, &current_moves))
    .collect();

    if lost_board.len() == 1 {
      return lost_board;
    }

    if acc.len() == 1 && is_win(&acc[0], &current_moves) && win_index == 0 {
      win_index = index;
    }
    acc
  });

  let moves_to_win: Vec<i32> = moves[..=win_index].to_vec();
  let flat_lost_board: Vec<i32> = flatten_board(&lost_board[0]);
  let unchecked = get_unchecked(flat_lost_board, moves_to_win);

  let uncheck_sum: i32 = unchecked.iter().sum();
  let last_called_val = moves[win_index];
  let multi = uncheck_sum * last_called_val;
  println!("{}", multi);
}