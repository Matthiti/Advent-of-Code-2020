pub mod part1 {
    use crate::days::input_parser;

    struct PasswordPolicy {
        min_bound: u32,
        max_bound: u32,
        character: char,
        password: String
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

        pub fn from(policy: String) -> Self {
            let parts: Vec<&str> = policy.split(' ').collect();

            let bounds: Vec<&str> = parts[0].split('-').collect();
            let min_bound = bounds[0].parse::<u32>().unwrap();
            let max_bound = bounds[1].parse::<u32>().unwrap();

            PasswordPolicy {
                min_bound,
                max_bound,
                character: parts[1].chars().next().unwrap(),
                password: parts[2].to_string()
            }
        }
    }

    pub fn start() -> u32 {
        let input = input_parser::parse_file::<String>("inputs/day02.txt").unwrap();

        let mut valid = 0;
        for i in input {
            let policy = PasswordPolicy::from(i);
            if policy.is_valid() {
                valid += 1;
            }
        }

        valid
    }
}

pub mod part2 {
    use crate::days::input_parser;

    struct OfficialPasswordPolicy {
        positions: (usize, usize),
        character: char,
        password: String
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

        pub fn from(policy: String) -> Self {
            let parts: Vec<&str> = policy.split(' ').collect();

            let bounds: Vec<&str> = parts[0].split('-').collect();
            let positions = (bounds[0].parse::<usize>().unwrap(), bounds[1].parse::<usize>().unwrap());

            OfficialPasswordPolicy {
                positions,
                character: parts[1].chars().next().unwrap(),
                password: parts[2].to_string()
            }
        }
    }

    pub fn start() -> u32 {
        let input = input_parser::parse_file::<String>("inputs/day02.txt").unwrap();

        let mut valid = 0;
        for i in input {
            let policy = OfficialPasswordPolicy::from(i);
            if policy.is_valid() {
                valid += 1;
            }
        }

        valid
    }
}