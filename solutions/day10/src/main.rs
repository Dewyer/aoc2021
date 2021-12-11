use std::collections::{HashMap, VecDeque};

const INPUT: &'static str = include_str!("../input.txt");
const REAL_INPUT: &'static str = include_str!("../real_input.txt");

fn get_closing_for(ch: char) -> char {
    match ch {
        '(' => ')',
        '[' => ']',
        '{' => '}',
        '<' => '>',
        _ => '-',
    }
}

fn get_point_for(ch: char) -> usize {
    match ch {
        ')' => 1,
        ']' => 2,
        '}' => 3,
        '>' => 4,
        _ => 0,
    }
}

fn solution() {
    let input = REAL_INPUT;

    let mut all_scores = vec![];

    for row in input.split('\n') {
        let mut score = 0;

        let mut chunk_que = VecDeque::<char>::new();
        let mut corrupted = false;
        for chr in row.chars() {
            let is_open = get_closing_for(chr) != '-';
            if is_open {
                chunk_que.push_front(chr);
            } else {
                let curr = chunk_que.pop_front();
                if let Some(cc) = curr {
                    if get_closing_for(cc.clone()) != chr {
                        // Corrupt
                        corrupted = true;
                        break;
                    }
                } else {
                    // illegal, close before open
                    corrupted = true;
                    break;
                }
            }
        }

        if corrupted {
            continue;
        }


        loop {
            if let Some(open_char) = chunk_que.pop_front() {
                let closing = get_closing_for(open_char);
                score *= 5;
                score += get_point_for(closing);
            }
            else {
                break;
            }
        }

        all_scores.push(score);
    }

    all_scores.sort_by(|aa, bb| aa.partial_cmp(bb).unwrap());
    let mid = all_scores.get((all_scores.len() - 1) / 2).unwrap();
    println!("{}", mid);
}

fn main() {
    solution();
}
