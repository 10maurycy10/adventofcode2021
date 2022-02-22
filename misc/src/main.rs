const NUMBERS: &str = "1234567890";

use nom::combinator::map_res;
use nom::multi::many0;
use nom::IResult;
use nom::bytes::complete::tag;
use nom::sequence::tuple;
use nom::multi::separated_list0;
use nom::bytes::complete::take_while;
use nom::Parser;

use nom::combinator::success;

fn sp<'a>(input: &'a str) -> IResult<&'a str, &'a str> {
    let chars = " \n\t\r";
    take_while(move |c: char| chars.contains(c))(input)
}

macro_rules! ws {
    ($x:expr) => { tuple((sp, $x, sp)).map(|x| x.1) }
}

fn parse_number<'a>(input: &'a str) -> IResult<&'a str, u32>  {
    map_res(take_while(|x| NUMBERS.contains(x)), |s: &str| s.parse::<u32>())(input)
}

fn parse_commalist<'a>(input: &'a str) -> IResult<&'a str, Vec<u32>>  {
    separated_list0(ws!(tag(",")), parse_number)(input)
}

fn parse_wslist<'a>(input: &'a str) -> IResult<&'a str, Vec<u32>>  {
    many0(ws!(parse_number))(input)
}

fn check_win(marked: &[[bool; 5]; 5]) -> bool {
    for x in 0..5 {
        let mut win = true;
        for y in 0..5 {
            win = win && marked[x][y];
        }
        if win {
            return true;
        }
    }
    for y in 0..5 {
        let mut win = true;
        for x in 0..5 {
            win = win && marked[x][y];
        }
        if win {
            return true;
        }
    }
    return false;
}

fn calc_score(numbers: &[[u32; 5]; 5], marked_spots: &[[bool; 5]; 5], called_number: u32) -> u32 {
    let mut unmarked_sum = 0;
    for x in 0..5 {
        for y in 0..5 {
            if !marked_spots[x][y] {
                unmarked_sum += numbers[x][y];
            }
        }
    }
    unmarked_sum * called_number
}

use std::io::BufRead;
fn main() {
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines();
    
    let winning: &str = &lines.next().unwrap().unwrap();
    let mut winning = parse_commalist(winning).unwrap().1;
    
    let lines: Vec<Vec<u32>> = lines.map(|x| x.unwrap()).map(|x| if (x.len() > 0) {parse_wslist(&x).unwrap().1} else {vec![]}).collect();
    
    let mut board_data: Vec<[[u32; 5]; 5]> = vec![];
    let mut board_idx = 0;
    let mut board_buffer = vec![];
    for line in lines {
        if line.len() != 0 {
            board_buffer.push(line.try_into().unwrap());
            if board_buffer.len() == 5 {
                board_data.push(board_buffer.try_into().unwrap());
                board_buffer = vec![];
            }
        } {
            board_idx += 1;
        }
    }
    
    let mut won_boards = vec![false; board_data.len()];
//     let mut won_boards_order = vec![];
    let mut marked_spots: Vec<[[bool; 5]; 5]> = vec![[[false;5];5]; board_data.len()];
    
    for number in winning.iter() {
        for (board_idx,board) in board_data.iter_mut().enumerate() {
            for (row_idx, row) in board.iter_mut().enumerate() {
                for (cell_idx, cell) in row.iter_mut().enumerate() {
                    if (*cell == *number) {
                        marked_spots[board_idx][row_idx][cell_idx] = true;
                    }
                }
            }
            if check_win(&marked_spots[board_idx]) {
                if !won_boards[board_idx] {
                    won_boards[board_idx] = true;
                    println!("board {} wins for {}", board_idx, calc_score(board, &marked_spots[board_idx], *number));
                }
            }
        }
    }
    
}
