use std::collections::HashMap;

const INPUT: &'static str = include_str!("../input.txt");
const REAL_INPUT: &'static str = include_str!("../real_input.txt");

fn flow_to(
    point: (usize, usize),
    board: &Vec<Vec<usize>>,
    lows: &Vec<(usize, usize)>,
) -> (usize, usize) {
    if let Some(low_p) =  lows.iter().find(|pp| pp.0 == point.0 &&  pp.1 == point.1) {
        return low_p.clone();
    }

    let dis: Vec<(i32, i32)> = vec![(0,1), (1,0), (0,-1), (-1,0)];

    let mut lowest = (usize::MAX, (0,0));
    for dir_vec in dis {
        let x_at = (point.0 as i32 + dir_vec.0) as usize;
        let oth_row = board.get((point.0 as i32 + dir_vec.0) as usize);
        if oth_row.is_none() {
            continue;
        }
        let y_at = (point.1 as i32 + dir_vec.1) as usize;
        let other = oth_row.unwrap().get((point.1 as i32 + dir_vec.1) as usize);
        if other.is_none() {
            continue;
        }
        let other = other.unwrap();

        if lowest.0 > *other {
            lowest = (*other, (x_at, y_at));
        }
    }

    return flow_to(lowest.1, board, lows);
}

fn to_key(val: &(usize, usize)) -> String {
    format!("{}-{}", val.0, val.1)
}

fn solution() {
    let input = REAL_INPUT;

    let mut board = vec![];

    for row in input.split('\n') {
        let vents = row.chars()
            .map(|el| usize::from_str_radix(&el.to_string(), 10).unwrap())
            .collect::<Vec<usize>>();
        board.push(vents);
    }

    let mut risk = 0;
    let mut lows = vec![];

    let dis: Vec<(i32, i32)> = vec![(0,1), (1,0), (0,-1), (-1,0)];
    for (row_ii, row) in board.iter().enumerate() {
        for (col_ii, el) in row.iter().enumerate() {
            let mut ok = true;
            for dir_vec in dis.iter() {
                let oth_row = board.get((row_ii as i32 + dir_vec.0) as usize);
                if oth_row.is_none() {
                    continue;
                }
                let other = oth_row.unwrap().get((col_ii as i32 + dir_vec.1) as usize);
                if other.is_none() {
                    continue;
                }
                let other = other.unwrap();

                if *other <= *el {
                    ok = false;
                    break;
                }
            }

            if ok {
                lows.push((row_ii, col_ii));
            }
        }
    }

    let mut sizes = HashMap::<String, usize>::new();

    for (row_ii, row) in board.iter().enumerate() {
        for (col_ii, el) in row.iter().enumerate() {
            if *el == 9 {
                continue;
            }

            let flow_to_pos = flow_to((row_ii, col_ii), &board, &lows);
            // println!("{:?}", flow_to_pos);
            let key = to_key(&flow_to_pos);
            if sizes.contains_key(&key) {
                *(sizes.get_mut(&key).unwrap()) += 1;
            } else {
                sizes.insert(key, 1);
            }
        }
    }

    let mut vec = sizes.clone().into_iter().collect::<Vec<(String, usize)>>();
    vec.sort_by(|aa, bb| aa.1.partial_cmp(&bb.1).unwrap().reverse() );

    let maxes = vec.into_iter().take(3).collect::<Vec<(String, usize)>>();
    let mut prod = 1;
    println!("{:?}", maxes);

    for max in maxes {
        prod *= max.1;
    }

    println!("{}", prod);
}

fn main() {
    solution();
}
