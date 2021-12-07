use std::collections::HashMap;

fn add_value_to_key(map: &mut HashMap<usize, usize>, key: usize, value: i64) {
    if let Some(entry) = map.get_mut(&key) {
        *entry = ((*entry as i64) + value) as usize;
    } else {
        map.insert(key, value.max(0) as usize);
    }
}

fn solution_a() {
    let input = include_str!("../input.txt");
    let timers = input.split(',')
        .map(|el| usize::from_str_radix(el, 10).unwrap())
        .collect::<Vec<usize>>();

    let until = 256;
    let mut fish = HashMap::<usize, usize>::new();
    for timer in timers.into_iter() {
        add_value_to_key(&mut fish, timer,1);
    }

    for _ in 0..until {
        let mut new_fish = HashMap::<usize, usize>::new();

        for (timer_group, fish_count) in fish.iter() {
            match timer_group {
                0 => {
                    add_value_to_key(&mut new_fish, 6, fish_count.clone() as i64);
                    add_value_to_key(&mut new_fish, 8, fish_count.clone() as i64);
                },
                _ => {
                    add_value_to_key(&mut new_fish, *timer_group - 1, fish_count.clone() as i64);
                }
            }
        }

        fish = new_fish;
    }

    let fish_count: usize = fish.values().sum();
    println!("{}", fish_count);
}

fn main() {
    solution_a();
}
