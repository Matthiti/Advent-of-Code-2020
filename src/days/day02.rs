pub mod part1 {
    use crate::days::input_parser;
    use std::str::FromStr;
    use std::string::ParseError;
    use std::num::ParseIntError;

    struct PasswordPolicy {
        min_bound: u32,
        max_bound: u32,
        character: char,
        password: String
    }

    impl FromStr for PasswordPolicy {
        type Err = ParseIntError;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let parts: Vec<&str> = s.split(' ').collect();

            let bounds: Vec<&str> = parts[0].split('-').collect();
            let min_bound = bounds[0].parse::<u32>()?;
            let max_bound = bounds[1].parse::<u32>()?;

            Ok(PasswordPolicy {
                min_bound,
                max_bound,
                character: parts[1].chars().next().unwrap(),
                password: parts[2].to_string()
            })
        }
    }

    impl PasswordPolicy {
        pub fn is_valid(&self) -> bool {
            let mut no_characters = 0;
            for c in self.password.chars() {
                if c == self.character {
                    no_characters += 1;
                }
            }

            no_characters >= self.min_bound && no_characters <= self.max_bound
        }
    }

    pub fn start() -> u32 {
        let input = input_parser::parse_file::<PasswordPolicy>("inputs/day02.txt").unwrap();

        let mut valid = 0;
        for policy in input {
            if policy.is_valid() {
                valid += 1;
            }
        }

        valid
    }
}

pub mod part2 {
    use crate::days::input_parser;
    use std::str::FromStr;
    use std::num::ParseIntError;

    struct OfficialPasswordPolicy {
        positions: (usize, usize),
        character: char,
        password: String
    }

    impl FromStr for OfficialPasswordPolicy {
        type Err = ParseIntError;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let parts: Vec<&str> = s.split(' ').collect();

            let bounds: Vec<&str> = parts[0].split('-').collect();
            let positions = (bounds[0].parse::<usize>()?, bounds[1].parse::<usize>()?);

            Ok(OfficialPasswordPolicy {
                positions,
                character: parts[1].chars().next().unwrap(),
                password: parts[2].to_string()
            })
        }
    }

    impl OfficialPasswordPolicy {
        pub fn is_valid(&self) -> bool {
            let first = self.password.chars().nth(self.positions.0 - 1);
            let second = self.password.chars().nth(self.positions.1 - 1);

            match (first, second) {
                (None, None) => false,
                (Some(first), None) => first == self.character,
                (None, Some(second)) => second == self.character,
                (Some(first), Some(second)) => (first == self.character) ^ (second == self.character)
            }
        }
    }

    pub fn start() -> u32 {
        let input = input_parser::parse_file::<OfficialPasswordPolicy>("inputs/day02.txt").unwrap();

        let mut valid = 0;
        for policy in input {
            if policy.is_valid() {
                valid += 1;
            }
        }

        valid
    }
}