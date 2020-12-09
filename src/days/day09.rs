pub mod part1 {
    use crate::days::input_parser;
    use std::collections::VecDeque;

    pub fn start() -> u64 {
        let input = input_parser::parse_file::<u64>("inputs/day09.txt").unwrap();

        let preamble_size = 25;
        let mut preamble = VecDeque::with_capacity(preamble_size);
        for i in 0..preamble_size {
            preamble.push_back(input[i]);
        }

        for i in preamble_size..input.len() {
            if !is_sum_of_preamble(input[i], &preamble) {
                return input[i];
            }
            preamble.pop_front();
            preamble.push_back(input[i]);
        }

        return 0; // Should never happen
    }

    fn is_sum_of_preamble(element: u64, preamble: &VecDeque<u64>) -> bool {
        for i in 0..preamble.len() {
            for j in i..preamble.len() {
                if preamble[i] + preamble[j] == element {
                    return true;
                }
            }
        }
        return false;
    }
}

pub mod part2 {
    use crate::days::input_parser;
    use std::collections::VecDeque;

    pub fn start() -> u64 {
        let input = input_parser::parse_file::<u64>("inputs/day09.txt").unwrap();

        let target = 18272118;
        for i in 2..input.len() {
            for j in 1..i {
                let mut k = 0;
                while j + k + i < input.len() {
                    let part = &input[j+k..j+k+i];
                    if part.iter().sum::<u64>() == target {
                        return part.iter().min().unwrap() + part.iter().max().unwrap();
                    }
                    k += i;
                }
            }
        }

        return 0; // Should never happen
    }
}