use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct CacheEntry {
    pub num_of_crabs_on: usize,
    pub fuel_total: usize,
}

fn get_left_cache(crab_positions: &Vec<usize>, max_pos: usize) -> Vec<CacheEntry> {
    let mut left_cache: Vec<CacheEntry> = vec![CacheEntry { num_of_crabs_on: 0, fuel_total: 0 }; max_pos+1];
    for crab_pos in crab_positions.iter() {
        let prev_val = left_cache.get(*crab_pos).unwrap();
        left_cache[*crab_pos] = CacheEntry { num_of_crabs_on: prev_val.num_of_crabs_on + 1, fuel_total: 0 };
    }

    for at_pos in 0..=max_pos {
        let mut cost_sum = 0;
        for pos in 0..at_pos {
            let to_go = at_pos - pos;
            let sum_to_go = (to_go * (to_go+1)) / 2;
            let cache_entr = left_cache.get(pos).unwrap();
            cost_sum += cache_entr.num_of_crabs_on * sum_to_go;
        }

        let prev_val = left_cache.get(at_pos).unwrap();
        left_cache[at_pos] = CacheEntry {
            num_of_crabs_on: prev_val.num_of_crabs_on,
            fuel_total: cost_sum,
        };

    }

    left_cache
}

fn get_right_cache(crab_positions: &Vec<usize>, max_pos: usize) -> Vec<CacheEntry> {
    let mut rev_crab = crab_positions.clone();
    rev_crab.reverse();
    // reverse numbers
    rev_crab = rev_crab.into_iter().map(|el| max_pos - el).collect();

    let mut cache = get_left_cache(&rev_crab, max_pos);
    cache.reverse();

    cache
}

fn solution() {
    let input = include_str!("../input.txt");
    let mut crab_positions = input.split(',')
        .map(|el| usize::from_str_radix(el, 10).unwrap())
        .collect::<Vec<usize>>();
    crab_positions.sort_by(|aa, bb| aa.partial_cmp(bb).unwrap());
    let max_pos = crab_positions.iter().max().unwrap().clone();

    println!("Sorted!");
    println!("{:?}", crab_positions);

    // Fuel required, num of crabs
    let left_cache = get_left_cache(&crab_positions, max_pos);
    let right_cache = get_right_cache(&crab_positions, max_pos);

    println!("{:?}", left_cache);
    println!("{:?}", right_cache);

    let mut min_cost = usize::MAX;
    let mut pos = 0;
    for position in 0..max_pos {
        let left_val = left_cache.get(position).unwrap();
        let right_val = right_cache.get(position).unwrap();
        let cost = left_val.fuel_total + right_val.fuel_total;

        if cost < min_cost {
            min_cost = cost;
            pos = position;
        }
    }

    println!("/====/");
    println!("{} at {}", min_cost, pos);
}

fn main() {
    solution();
}
