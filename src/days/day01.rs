pub mod part1 {
    use crate::days::input_parser;

    pub fn start() -> i32 {
        let input = input_parser::parse_file::<i32>("inputs/day01.txt").unwrap();

        for i in 0..input.len() {
            for j in i+1..input.len() {
                if input[i] + input[j] == 2020 {
                    return input[i] * input[j];
                }
            }
        }
        // Should never happen
        return -1;
    }
}

pub mod part2 {
    use crate::days::input_parser;

    pub fn start() -> i32 {
        let input = input_parser::parse_file::<i32>("inputs/day01.txt").unwrap();

        for i in 0..input.len() {
            for j in i+1..input.len() {
                for k in j+1..input.len() {
                    if input[i] + input[j] + input[k] == 2020 {
                        return input[i] * input[j] * input[k];
                    }
                }
            }
        }
        // Should never happen
        return -1;
    }
}