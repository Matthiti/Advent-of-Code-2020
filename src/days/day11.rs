pub mod part1 {
    use crate::days::input_parser;
    use core::fmt;
    use std::fmt::Formatter;

    #[derive(PartialEq)]
    #[derive(Clone)]
    enum WaitingPosition {
        EmptySeat,
        OccupiedSeat,
        Floor
    }

    impl fmt::Display for WaitingPosition {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            write!(f, "{}", match self {
                WaitingPosition::EmptySeat => "L",
                WaitingPosition::OccupiedSeat => "#",
                WaitingPosition::Floor => "."
            })
        }
    }

    impl WaitingPosition {
        fn from(s: char) -> Result<Self, String> {
            match s {
                'L' => Ok(WaitingPosition::EmptySeat),
                '#' => Ok(WaitingPosition::OccupiedSeat),
                '.' => Ok(WaitingPosition::Floor),
                _   => Err(String::from("Invalid waiting position"))
            }
        }
    }

    struct SeatLayout {
        grid: Vec<Vec<WaitingPosition>>
    }

    impl fmt::Display for SeatLayout {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            for row in &self.grid {
                for pos in row {
                    write!(f, "{}", pos)?;
                }
                write!(f, "\n")?;
            }
            Ok(())
        }
    }

    impl SeatLayout {
        fn new(grid: Vec<Vec<WaitingPosition>>) -> SeatLayout {
            SeatLayout { grid }
        }

        fn adjacent_occupied_seats(&self, row: usize, column: usize) -> usize {
            let result = [
                // Up
                row > 0 && self.grid[row - 1][column] == WaitingPosition::OccupiedSeat,
                // Down
                row + 1 < self.grid.len() && self.grid[row + 1][column] == WaitingPosition::OccupiedSeat,
                // Left
                column > 0 && self.grid[row][column - 1] == WaitingPosition::OccupiedSeat,
                // Right
                column + 1 < self.grid[row].len() && self.grid[row][column + 1] == WaitingPosition::OccupiedSeat,
                // Diagonal up left
                row > 0 && column > 0 && self.grid[row - 1][column - 1] == WaitingPosition::OccupiedSeat,
                // Diagonal up right
                row > 0 && column + 1 < self.grid[row].len() && self.grid[row - 1][column + 1] == WaitingPosition::OccupiedSeat,
                // Diagonal down left
                row + 1 < self.grid.len() && column > 0 && self.grid[row + 1][column - 1] == WaitingPosition::OccupiedSeat,
                // Diagonal down right
                row + 1 < self.grid.len() && column + 1 < self.grid[row].len() && self.grid[row + 1][column + 1] == WaitingPosition::OccupiedSeat
            ];

            result.iter().filter(|v| **v).count()
        }
    }

    pub fn start() -> u32 {
        let input = input_parser::parse_file::<String>("inputs/day11.txt").unwrap();

        let mut positions = vec![];
        for row in input {
            let mut r = vec![];
            for column in row.chars() {
                r.push(WaitingPosition::from(column).unwrap());
            }
            positions.push(r);
        }

        let mut layout = SeatLayout::new(positions);

        let mut changed = true;
        while changed {
            changed = false;
            let mut new_grid = vec![];
            for i in 0..layout.grid.len() {
                new_grid.push(vec![]);
                for j in 0..layout.grid[i].len() {
                    if layout.grid[i][j] == WaitingPosition::EmptySeat && layout.adjacent_occupied_seats(i, j) == 0 {
                        new_grid[i].push(WaitingPosition::OccupiedSeat);
                        changed = true;
                    } else if layout.grid[i][j] == WaitingPosition::OccupiedSeat && layout.adjacent_occupied_seats(i, j) >= 4 {
                        new_grid[i].push(WaitingPosition::EmptySeat);
                        changed = true;
                    } else {
                        new_grid[i].push(layout.grid[i][j].clone());
                    }
                }
            }
            layout.grid = new_grid;
        }

        let mut result = 0;
        for row in layout.grid {
            for pos in row {
                if pos == WaitingPosition::OccupiedSeat {
                    result += 1;
                }
            }
        }

        result
    }
}

pub mod part2 {
    use crate::days::input_parser;
    use core::fmt;
    use std::fmt::Formatter;

    #[derive(PartialEq)]
    #[derive(Clone)]
    enum WaitingPosition {
        EmptySeat,
        OccupiedSeat,
        Floor
    }

    impl fmt::Display for WaitingPosition {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            write!(f, "{}", match self {
                WaitingPosition::EmptySeat => "L",
                WaitingPosition::OccupiedSeat => "#",
                WaitingPosition::Floor => "."
            })
        }
    }

    impl WaitingPosition {
        fn from(s: char) -> Result<Self, String> {
            match s {
                'L' => Ok(WaitingPosition::EmptySeat),
                '#' => Ok(WaitingPosition::OccupiedSeat),
                '.' => Ok(WaitingPosition::Floor),
                _   => Err(String::from("Invalid waiting position"))
            }
        }
    }

    struct SeatLayout {
        grid: Vec<Vec<WaitingPosition>>
    }

    impl fmt::Display for SeatLayout {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            for row in &self.grid {
                for pos in row {
                    write!(f, "{}", pos)?;
                }
                write!(f, "\n")?;
            }
            Ok(())
        }
    }

    impl SeatLayout {
        fn new(grid: Vec<Vec<WaitingPosition>>) -> SeatLayout {
            SeatLayout { grid }
        }

        fn visible_occupied_seats(&self, row: usize, column: usize) -> usize {
            let result = [
                self.occupied_left(row, column),
                self.occupied_right(row, column),
                self.occupied_up(row, column),
                self.occupied_down(row, column),
                self.occupied_diagonal_up_left(row, column),
                self.occupied_diagonal_up_right(row, column),
                self.occupied_diagonal_down_left(row, column),
                self.occupied_diagonal_down_right(row, column)
            ];

            result.iter().filter(|v| **v).count()
        }

        fn occupied_up(&self, mut row: usize, column: usize) -> bool {
            while row > 0 {
                row -= 1;
                match self.grid[row][column] {
                    WaitingPosition::OccupiedSeat => return true,
                    WaitingPosition::EmptySeat => return false,
                    WaitingPosition::Floor => continue
                }
            }
            false
        }

        fn occupied_down(&self, mut row: usize, column: usize) -> bool {
            while row + 1 < self.grid.len() {
                row += 1;
                match self.grid[row][column] {
                    WaitingPosition::OccupiedSeat => return true,
                    WaitingPosition::EmptySeat => return false,
                    WaitingPosition::Floor => continue
                }
            }
            false
        }

        fn occupied_left(&self, row: usize, mut column: usize) -> bool {
            while column > 0 {
                column -= 1;
                match self.grid[row][column] {
                    WaitingPosition::OccupiedSeat => return true,
                    WaitingPosition::EmptySeat => return false,
                    WaitingPosition::Floor => continue
                }
            }
            false
        }

        fn occupied_right(&self, row: usize, mut column: usize) -> bool {
            while column + 1 < self.grid[row].len() {
                column += 1;
                match self.grid[row][column] {
                    WaitingPosition::OccupiedSeat => return true,
                    WaitingPosition::EmptySeat => return false,
                    WaitingPosition::Floor => continue
                }
            }
            false
        }

        fn occupied_diagonal_up_left(&self, mut row: usize, mut column: usize) -> bool {
            while row > 0 && column > 0 {
                row -= 1;
                column -= 1;
                match self.grid[row][column] {
                    WaitingPosition::OccupiedSeat => return true,
                    WaitingPosition::EmptySeat => return false,
                    WaitingPosition::Floor => continue
                }
            }
            false
        }

        fn occupied_diagonal_up_right(&self, mut row: usize, mut column: usize) -> bool {
            while row > 0 && column + 1 < self.grid[row].len() {
                row -= 1;
                column += 1;
                match self.grid[row][column] {
                    WaitingPosition::OccupiedSeat => return true,
                    WaitingPosition::EmptySeat => return false,
                    WaitingPosition::Floor => continue
                }
            }
            false
        }

        fn occupied_diagonal_down_left(&self, mut row: usize, mut column: usize) -> bool {
            while row + 1 < self.grid.len() && column > 0 {
                row += 1;
                column -= 1;
                match self.grid[row][column] {
                    WaitingPosition::OccupiedSeat => return true,
                    WaitingPosition::EmptySeat => return false,
                    WaitingPosition::Floor => continue
                }
            }
            false
        }

        fn occupied_diagonal_down_right(&self, mut row: usize, mut column: usize) -> bool {
            while row + 1 < self.grid.len() && column + 1 < self.grid[row].len() {
                row += 1;
                column += 1;
                match self.grid[row][column] {
                    WaitingPosition::OccupiedSeat => return true,
                    WaitingPosition::EmptySeat => return false,
                    WaitingPosition::Floor => continue
                }
            }
            false
        }
    }

    pub fn start() -> u32 {
        let input = input_parser::parse_file::<String>("inputs/day11.txt").unwrap();

        let mut positions = vec![];
        for row in input {
            let mut r = vec![];
            for column in row.chars() {
                r.push(WaitingPosition::from(column).unwrap());
            }
            positions.push(r);
        }

        let mut layout = SeatLayout::new(positions);

        let mut changed = true;
        while changed {
            changed = false;
            let mut new_grid = vec![];
            for i in 0..layout.grid.len() {
                new_grid.push(vec![]);
                for j in 0..layout.grid[i].len() {
                    if layout.grid[i][j] == WaitingPosition::EmptySeat && layout.visible_occupied_seats(i, j) == 0 {
                        new_grid[i].push(WaitingPosition::OccupiedSeat);
                        changed = true;
                    } else if layout.grid[i][j] == WaitingPosition::OccupiedSeat && layout.visible_occupied_seats(i, j) >= 5 {
                        new_grid[i].push(WaitingPosition::EmptySeat);
                        changed = true;
                    } else {
                        new_grid[i].push(layout.grid[i][j].clone());
                    }
                }
            }
            layout.grid = new_grid;
        }

        let mut result = 0;
        for row in layout.grid {
            for pos in row {
                if pos == WaitingPosition::OccupiedSeat {
                    result += 1;
                }
            }
        }

        result
    }
}