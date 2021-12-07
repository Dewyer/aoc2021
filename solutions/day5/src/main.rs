pub type Coords = (usize, usize);

#[derive(Debug)]
pub struct Line {
    pub id: usize,
    pub from: Coords,
    pub to: Coords,
}

impl Line {
    fn coords_from_str(str_coord: &str) -> Coords {
        let mut tks = str_coord
            .split(',')
            .map(|el| usize::from_str_radix(el, 10).expect("NaN!"));

        (tks.next().unwrap(),tks.next().unwrap())
    }

    pub fn from_string(id: usize, str_line: &str) -> Self {
        let mut coord_tokens = str_line.split(" -> ");
        let first = coord_tokens.next().expect("Expected first coord.");
        let second = coord_tokens.next().expect("Expected second coord.");

        Self {
            id,
            from: Self::coords_from_str(first),
            to: Self::coords_from_str(second),
        }
    }

    pub fn is_straight(&self) -> bool {
        self.from.0 == self.to.0 || self.from.1 == self.to.1
    }

    pub fn get_touched_points(&self) -> Vec<Coords> {
        if self.is_straight() {
            self.get_touching_points_straight()
        } else {
            self.get_touching_points_diagonal()
        }
    }

    fn get_touching_points_diagonal(&self) -> Vec<Coords> {
        let ordered = {
            if self.from.0 < self.to.0 {
                (self.from, self.to)
            } else {
                (self.to, self.from)
            }
        };

        let slope = {
            if ordered.0.1 < ordered.1.1 {
                1i32
            } else {
                -1i32
            }
        };
        let mut coords = vec![];

        for (index, xx) in (ordered.0.0..=ordered.1.0).enumerate() {
            coords.push((xx, (ordered.0.1 as i32 + (index as i32 * slope)) as usize));
        }

        coords
    }

    fn get_touching_points_straight(&self) -> Vec<Coords> {
        let mut coords = vec![];

        let ll = self.from.0.max(self.to.0);
        let rr = self.from.0.min(self.to.0);

        let aa = self.from.1.max(self.to.1);
        let bb = self.from.1.min(self.to.1);

        for xx in rr..=ll {
            for yy in bb..=aa {
                coords.push((xx, yy));
            }
        }

        coords
    }

    pub fn max_coordinate(&self) -> usize {
        self.from.0
            .max(self.from.1.max(self.to.0.max(self.to.1)))
    }
}

fn solve_a() {
    let input = include_str!("../real_input.txt");
    let lines = input.split('\n')
        .enumerate()
        .map(|(ii, row)| Line::from_string(ii, row))
        .collect::<Vec<Line>>();
    let mut board_size = 0;

    for ll in lines.iter() {
        let max = ll.max_coordinate() + 1;
        if max >= board_size {
            board_size = max;
        }
    }

    let mut board = vec![vec![0usize;board_size];board_size];
    for line in lines.iter() {
        let touches = line.get_touched_points();
        for touched in touches.into_iter() {
            board[touched.1][touched.0] += 1;
        }
    }

    let mut intersections = 0;

    for row in board.iter() {
        for el in row.iter() {
            if *el > 1usize {
                intersections += 1;
            }
        }
    }

    println!("{}", intersections);
}

fn main() {
    solve_a();
}
