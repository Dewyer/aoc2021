use std::collections::{HashMap, VecDeque, HashSet};

const INPUT: &'static str = include_str!("../input.txt");
const REAL_INPUT: &'static str = include_str!("../real_input.txt");

type MapType = HashMap<String, Vec<String>>;

fn add_to(map: &mut MapType, from: &str, to: &str) {
    if let Some(map_el) = map.get_mut(from) {
        map_el.push(to.to_string());
    } else {
        map.insert(from.to_string(), vec![to.to_string()]);
    }
}

fn is_small_cave(cave: &str) ->bool {
    cave.to_lowercase() == cave
}

struct Problem {
    pub at: String,
    pub visited_smalls: HashSet<String>,
    pub double_up: Option<String>,
    pub path : Vec<String>,
}

impl Problem {
    pub fn add_path_el(&self, cave: &str) -> Self {
        let mut new_vis = self.visited_smalls.clone();
        let mut dup = self.double_up.clone();
        if is_small_cave(cave) {
            if new_vis.contains(cave) && dup.is_none() {
                dup = Some(cave.to_string());
            } else {
                new_vis.insert(cave.to_string());
            }
        }
        let mut n_path = self.path.clone();
        n_path.push(cave.to_string());

        Self {
            at: cave.to_string(),
            visited_smalls: new_vis,
            path: n_path,
            double_up: dup,
        }
    }

    pub fn can_go_to_cave(&self, cave: &str) -> bool {
        if is_small_cave(cave) {
            if cave == "start" || cave == "end" {
                !self.visited_smalls.contains(cave)
            } else {
                !self.visited_smalls.contains(cave) || self.double_up.is_none()
            }
        } else {
            true
        }
    }
}

fn solution(inp: &str) {
    let mut map = HashMap::<String, Vec<String>>::new();

    for row in inp.split('\n') {
        let mut parts = row.split('-');
        let from = parts.next().unwrap();
        let to = parts.next().unwrap();

        add_to(&mut map, from ,to);
        add_to(&mut map, to ,from);
    }

    let mut pc = 0;

    let mut stack = VecDeque::new();
    let mut hsh = HashSet::new();
    hsh.insert("start".to_string());

    stack.push_front(Problem {
        at: "start".to_string(),
        visited_smalls: hsh,
        path: vec!["start".to_string()],
        double_up: None,
    });

    while stack.len() != 0 {
        let problem = stack.pop_front().unwrap();
        let def = vec![];
        let can_go_from_here = map.get(&problem.at).unwrap_or(&def);

        for next_cave in can_go_from_here.iter() {
            if !problem.can_go_to_cave(next_cave) {
                continue;
            }
            let np = problem.add_path_el(next_cave);

            if next_cave == "end" {
                // println!("{:?}", np.path);
                pc += 1;
                continue;
            }

            stack.push_front(np);
        }
    }

    println!("{}", pc);
}

fn main() {
    solution(REAL_INPUT);
}
