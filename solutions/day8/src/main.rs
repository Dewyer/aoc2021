use std::collections::{HashMap, HashSet};
use std::ops::Index;

const INPUT: &'static str = include_str!("../input.txt");
const REAL_INPUT: &'static str = include_str!("../real_input.txt");

fn solution() {
    let input = REAL_INPUT;
    let input = input.split('\n');
    let mut cc = 0;

    for row in input {
        let left = row.split('|').next().unwrap();
        let right = row.split('|').skip(1).next().unwrap();
        println!("{}", right);
        let tokens = right.split(' ')
            .map(|el| el.len())
            .collect::<Vec<usize>>();

        println!("{:?}", tokens);
        for tt in tokens {
            if tt == 2 || tt == 3 || tt == 4 || tt == 7 {
                cc += 1
            }
        }
    }

    println!("{}", cc);

}

fn subsit(
    cypher: &str,
    mapping: &HashMap<char, char>,
) -> String {
    let mut mapped = "".to_string();
    let mut incomp = false;

    for ch in cypher.chars() {
        let new_char = mapping.get(&ch);
        if new_char.is_none() {
            incomp = true;
            break;
        }

        let nc = new_char.unwrap();
        mapped += &nc.to_string();
    }

    if incomp {
        return "".to_string();
    }

    let mut mapped = mapped.chars().collect::<Vec<char>>();
    mapped.sort_by(|aa,bb| aa.partial_cmp(bb).unwrap());

    let mut kek = "".to_string();
    for rr in mapped.into_iter() {
        kek += &rr.to_string()
    }

    kek
}

fn is_sol_not_bad(
    mapping: &HashMap<char, char>,
    signals: &Vec<String>,
) -> bool {
    let VALIDS: Vec<&'static str> = vec![
        "abcefg",
        "cf",
        "acdeg",
        "acdfg",
        "bcdf",
        "abdfg",
        "abdefg",
        "acf",
        "abcdefg",
        "abcdfg",
    ];

    let mut seen = HashSet::new();
    for sig in signals {
        let mut mapped = subsit(sig, mapping);
        if mapped.len() == 0 {
           continue;
        }

        let no = VALIDS.iter().enumerate().find(|el| el.1.clone() == mapped);
        if let Some(kek) = no {
            if seen.contains(&kek.0) {
                return false;
            }
             else {
                 seen.insert(kek.0);
             }
        } else {
            return false;
        }
    }

    true
}

fn back_track_sol(
    at: usize,
    mapping: &HashMap<char, char>,
    signals: &Vec<String>,
) -> (bool, Option<HashMap<char, char>>) {
    let order = "abcdefg";

    // check if prev ok, any combo gives any invalid on signals
    let is_n_bad = is_sol_not_bad(mapping, signals);
    if !is_n_bad {
        return (false, None);
    }

    if at == 7 {
        return (true, Some(mapping.clone()));
    }

    let mfor = order.chars().skip(at).next().unwrap();

    for ii in order.chars() {
        let mut new_map = mapping.clone();
        new_map.insert(mfor, ii);

        let can_be = back_track_sol(at+1, &new_map, &signals);
        if can_be.0 {
            return can_be;
        }
    }

    (false, None)
}

fn sol() {
    let VALIDS: Vec<&'static str> = vec![
        "abcefg",
        "cf",
        "acdeg",
        "acdfg",
        "bcdf",
        "abdfg",
        "abdefg",
        "acf",
        "abcdefg",
        "abcdfg",
    ];

    let input = REAL_INPUT;
    let input = input.split('\n');
    let mut cc = 0;
    for row in input {
        let left = row.split('|').next().unwrap().split(' ').map(|el| el.to_string()).collect::<Vec<String>>();
        let btr = back_track_sol(0, &HashMap::new(), &left);

        let (ii, kk) = btr;
        if !ii {
            panic!("WWAA");
        }
        let aa = kk.unwrap();
        // println!("{:?}", aa);

        let right = row.split('|').skip(1).next().unwrap().split(' ').map(|el| el.to_string()).collect::<Vec<String>>();
        let mut st = "".to_string();
        for od in right.into_iter() {
            if od.len() == 0 {
                continue;
            }
            let decyp = subsit(&od, &aa);
            let no = VALIDS.iter().enumerate().find(|el| el.1.clone() == decyp).unwrap().0;
            // println!("{}", no);

            st += &no.to_string();
        }

        cc += usize::from_str_radix(&st, 10).unwrap();
        // println!("====");
    }

    println!("{}", cc);
}

fn main() {
    sol();
}
