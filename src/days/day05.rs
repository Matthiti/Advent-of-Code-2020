pub mod part1 {
    use crate::days::input_parser;
    use std::cmp::max;

    pub fn start() -> u32 {
        let input = input_parser::parse_file::<String>("inputs/day05.txt").unwrap();

        let mut max_seat = 0;
        for boarding_pass in input.iter() {
            // row
            let mut min_row = 0;
            let mut max_row = 127;
            for i in 0..7 {
                match boarding_pass.chars().nth(i) {
                    Some('F') => max_row = (min_row + max_row) / 2,
                    Some('B') => min_row = (min_row + max_row + 1) / 2,
                    _ => eprintln!("Invalid boarding pass")
                }
            }

            assert_eq!(min_row, max_row);
            let row = min_row;

            // column
            let mut min_col = 0;
            let mut max_col = 7;
            for i in 7..10 {
                match boarding_pass.chars().nth(i) {
                    Some('L') => max_col = (min_col + max_col) / 2,
                    Some('R') => min_col = (min_col + max_col + 1) / 2,
                    _ => eprintln!("Invalid boarding pass")
                }
            }

            assert_eq!(min_col, max_col);
            let col = min_col;

            max_seat = max(max_seat, row * 8 + col);
        }

        max_seat
    }
}

pub mod part2 {
    use crate::days::input_parser;

    pub fn start() -> i32 {
        let input = input_parser::parse_file::<String>("inputs/day05.txt").unwrap();

        let mut ids = vec![];
        for boarding_pass in input.iter() {
            // row
            let mut min_row = 0;
            let mut max_row = 127;
            for i in 0..7 {
                match boarding_pass.chars().nth(i) {
                    Some('F') => max_row = (min_row + max_row) / 2,
                    Some('B') => min_row = (min_row + max_row + 1) / 2,
                    _ => eprintln!("Invalid boarding pass")
                }
            }

            assert_eq!(min_row, max_row);
            let row = min_row;

            if row == 0 || row == 127 {
                continue;
            }

            // column
            let mut min_col = 0;
            let mut max_col = 7;
            for i in 7..10 {
                match boarding_pass.chars().nth(i) {
                    Some('L') => max_col = (min_col + max_col) / 2,
                    Some('R') => min_col = (min_col + max_col + 1) / 2,
                    _ => eprintln!("Invalid boarding pass")
                }
            }

            assert_eq!(min_col, max_col);
            let col = min_col;

            ids.push(row * 8 + col);
        }

        ids.sort();

        for i in 0..ids.len() - 1 {
            if ids[i] == ids[i + 1] - 2 {
                return ids[i] + 1;
            }
        }

        -1
    }
}