pub mod part1 {
    use crate::days::input_parser;

    pub fn start() -> u32 {
        let input = input_parser::parse_file::<String>("inputs/day03.txt").unwrap();

        let width = input[0].len();
        let mut pos_x = 0;
        let mut result = 0;

        for line in input.iter() {
            if line.chars().nth(pos_x % width).unwrap() == '#' {
                result += 1;
            }
            pos_x += 3;
        }

        result
    }
}

pub mod part2 {
    use crate::days::input_parser;

    pub fn start() -> u32 {
        let input = input_parser::parse_file::<String>("inputs/day03.txt").unwrap();

        let width = input[0].len();
        let height = input.len();

        let mut result = 1;
        let slopes = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

        for slope in slopes.iter() {
            let mut sum = 0;

            let mut pos_x = 0;
            let mut pos_y = 0;

            while pos_y < height {
                if input[pos_y].chars().nth(pos_x % width).unwrap() == '#' {
                    sum += 1;
                }
                pos_x += slope.0;
                pos_y += slope.1;
            }

            result *= sum;
        }

        result
    }
}