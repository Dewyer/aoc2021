use std::collections::{HashMap, VecDeque, HashSet};

const INPUT: &'static str = include_str!("../input.txt");
const REAL_INPUT: &'static str = include_str!("../real_input.txt");

fn to_key(row: usize, col: usize) -> String {
    format!("{}-{}", row, col)
}

fn bump(dumbos:&mut Vec<Vec<usize>>) {
    for row in dumbos.iter_mut() {
        for el in row.iter_mut() {
            *el += 1;
        }
    }
}

fn flash(
    dumbos:&mut Vec<Vec<usize>>,
    flash_set:&mut HashSet<String>,
) -> usize {
    let size = dumbos.len();
    let mut flash_count = 0;

    for row_ii in 0..size {
        for col_ii in 0..size {
            let val = dumbos.get(row_ii).unwrap().get(col_ii).unwrap();
            let key = to_key(row_ii, col_ii);
            if *val <= 9 || flash_set.contains(&key) {
                continue;
            }

            flash_set.insert(key);
            flash_count += 1;
            for xx_diff in -1i32..=1 {
                for yy_diff in -1i32..=1 {
                    if xx_diff == 0 && yy_diff == 0 {
                        continue;
                    }

                    let next = dumbos.get_mut((row_ii as i32 + xx_diff) as usize);
                    if let Some(row) = next {
                        let col_el = row.get_mut((col_ii as i32 + yy_diff) as usize);

                        if let Some(col) = col_el {
                            *col += 1;
                        }
                    }
                }
            }
        }
    }

    flash_count
}

fn solution(inp: &str) {
    let mut dumbos = vec![];

    for row in inp.split('\n') {
        let mut row_dum = row.chars()
            .map(|el| usize::from_str_radix(&el.to_string(), 10).unwrap())
            .collect::<Vec<usize>>();

        dumbos.push(row_dum);
    }

    let mut that = 0;

    for step in 1..100_00000 {
        let mut flash_set = HashSet::new();
        bump(&mut dumbos);

        loop {
            let flash_cnt = flash(&mut dumbos, &mut flash_set);
            if flash_cnt == 0 {
                break;
            }

            // total += flash_cnt;
        }

        if flash_set.len()  == 100 {
            that = step;
            break;
        }

        for kk in flash_set.into_iter() {
            let cords = kk.split('-')
                .map(|el| usize::from_str_radix(el, 10).unwrap())
                .collect::<Vec<usize>>();
            let el = dumbos.get_mut(cords[0]).unwrap().get_mut(cords[1]).unwrap();
            *el = 0;
        }
    }

    println!("{}", that);
}

fn main() {
    solution(REAL_INPUT);
}
