use crate::bingo_board::BingoBoard;
use std::collections::{HashSet};

pub mod bingo_board;

fn solve_a() {
    let input = include_str!("../real_input.txt");
    let mut sections = input.split("\n\n");
    let draws = sections.next().expect("No firt section!");
    let mut boards = vec![];

    for section in sections {
        boards.push(BingoBoard::from_text_section(section));
    }

    let draw_nums = draws
        .split(',')
        .filter(|el| el.trim().len() != 0)
        .map(|el| i32::from_str_radix(el, 10).expect("Expected number!"));

    let mut winner_board_score = None;

    for draw in draw_nums {
        for board in boards.iter_mut() {
            let did_win = board.check_off(draw);
            if did_win {
                winner_board_score = Some(board.get_sum_of_unmarked() * draw);
                break;
            }
        }

        if winner_board_score.is_some() {
            break;
        }
    }

    let winner_board_score = winner_board_score.expect("Expected someone to win!");

    println!("{}", winner_board_score);
}

fn solve_b() {
    let input = include_str!("../real_input.txt");
    let mut sections = input.split("\n\n");
    let draws = sections.next().expect("No firt section!");
    let mut boards = vec![];

    for section in sections {
        boards.push(BingoBoard::from_text_section(section));
    }

    let draw_nums = draws
        .split(',')
        .filter(|el| el.trim().len() != 0)
        .map(|el| i32::from_str_radix(el, 10).expect("Expected number!"));

    let mut winners = HashSet::new();
    let mut winner_count = 0;
    let board_count = boards.len();

    for draw in draw_nums {
        for (ii, board) in boards.iter_mut().enumerate() {
            let did_win = board.check_off(draw);
            if did_win && !winners.contains(&ii) {
                winners.insert(ii);
                winner_count += 1;

                if winner_count == board_count {
                    println!("{}", board.get_sum_of_unmarked() * draw);
                    return;
                }
            }
        }
    }
}

fn main() {
    solve_b();
}
