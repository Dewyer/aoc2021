use std::borrow::Borrow;

pub struct BingoBoard {
    board: Vec<Vec<(bool, i32)>>,
}

impl BingoBoard {
    pub fn from_text_section(section: &str) -> Self {
        let rows = section.split('\n');
        let mut board = vec![];

        for row in rows {
            let mut board_row = vec![];
            for row_el in row.split(' ') {
                if row_el.trim().len() == 0 {
                    continue;
                }

                let num = i32::from_str_radix(row_el, 10).expect("Expected number!");
                board_row.push((false, num));
            }

            board.push(board_row);
        }

        Self {
            board,
        }
    }

    fn is_winning_with_rows(&self) -> bool {
        for row in self.board.iter() {
            let mut row_ok = true;
            for el in row.iter() {
                if !el.0 {
                    row_ok = false;
                    break;
                }
            }

            if row_ok {
                return true;
            }
        }

        false
    }

    fn is_winning_with_cols(&self) -> bool {
        let size = self.board.len();
        for col_index in 0..size {
            let mut row_ok = true;

            for row_index in 0..size {
                let el = self.board[row_index][col_index].borrow();
                if !el.0 {
                    row_ok = false;
                    break;
                }
            }

            if row_ok {
                return true;
            }
        }

        false
    }

    pub fn is_winning(&self) -> bool {
        let rows_ok = self.is_winning_with_rows();
        let cols_ok = self.is_winning_with_cols();

        rows_ok || cols_ok
    }

    pub fn check_off(&mut self, num: i32) -> bool {
        for row in self.board.iter_mut() {
            for el in row.iter_mut() {
                if el.1 == num {
                    el.0 = true;
                }
            }
        }

        self.is_winning()
    }

    pub fn get_sum_of_unmarked(&self) -> i32 {
        let mut sum_unmarked = 0;
        for row in self.board.iter() {
            for el in row.iter() {
                if !el.0 {
                    sum_unmarked += el.1;
                }
            }
        }

        sum_unmarked
    }
}