pub mod part1 {
    use crate::days::input_parser;
    use std::collections::HashSet;

    pub fn start() -> u32 {
        let input = input_parser::parse_file_raw("inputs/day06.txt").unwrap();
        let groups: Vec<&str> = input.split("\n\n").collect();

        let mut result = 0;
        for group in &groups {
            let answers = group.split_whitespace().collect::<Vec<&str>>().join("");
            result += unique_chars(&answers);
        }

        result
    }

    fn unique_chars(s: &str) -> u32 {
        s.chars().collect::<HashSet<char>>().len() as u32
    }
}

pub mod part2 {
    use crate::days::input_parser;
    use std::collections::HashMap;

    pub fn start() -> u32 {
        let input = input_parser::parse_file_raw("inputs/day06.txt").unwrap();
        let groups: Vec<&str> = input.split("\n\n").collect();

        let mut result = 0;
        for answers in &groups {
            let answers_per_person: Vec<&str> = answers.split("\n").collect();
            result += same_chars(answers_per_person);
        }

        result
    }

    fn same_chars(vec: Vec<&str>) -> u32 {
        let mut occurrences: HashMap<char, u32> = HashMap::new();
        for s in &vec {
            for c in s.chars() {
                occurrences.insert(c, *occurrences.get(&c).unwrap_or(&0) + 1);
            }
        }

        occurrences.iter().fold(0, |acc, (_, x) | if x == &(vec.len() as u32) { acc + 1 } else { acc })
    }
}