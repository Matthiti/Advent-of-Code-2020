pub mod part1 {
    use crate::days::input_parser;

    pub fn start() -> u32 {
        let input = input_parser::parse_file_raw("inputs/day04.txt").unwrap();
        let passports: Vec<&str> = input.split("\n\n").collect();

        let required_attributes = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

        let mut result = 0;
        for passport in passports.iter() {
            let attributes: Vec<&str> = passport.split_whitespace().collect();
            let mut valid = true;
            for attribute in required_attributes.iter() {
                if !attributes.iter().any(|&a| a.starts_with(attribute)) {
                    valid = false;
                }
            }

            if valid {
                result += 1;
            }
        }

        result
    }
}

pub mod part2 {
    use crate::days::input_parser;

    pub fn start() -> u32 {
        let input = input_parser::parse_file_raw("inputs/day04.txt").unwrap();
        let passports: Vec<&str> = input.split("\n\n").collect();

        let required_attributes: [(&str, fn(&str) -> bool); 7] = [
            ("byr", |value: &str| {
                if let Ok(year) = value.parse::<u32>() {
                    year >= 1920 && year <= 2002
                } else {
                    false
                }
            }),
            ("iyr", |value: &str| {
                if let Ok(year) = value.parse::<u32>() {
                    year >= 2010 && year <= 2020
                } else {
                    false
                }
            }),
            ("eyr", |value: &str| {
                if let Ok(year) = value.parse::<u32>() {
                    year >= 2020 && year <= 2030
                } else {
                    false
                }
            }),
            ("hgt", |value: &str| {
                if value.ends_with("cm") {
                    if let Ok(height) = value.strip_suffix("cm").unwrap().parse::<u32>() {
                        height >= 150 && height <= 193
                    } else {
                        false
                    }
                } else if value.ends_with("in") {
                    if let Ok(height) = value.strip_suffix("in").unwrap().parse::<u32>() {
                        height >= 59 && height <= 76
                    } else {
                        false
                    }
                } else {
                    false
                }
            }),
            ("hcl", |value: &str| {
                if !value.starts_with("#") {
                    return false;
                }

                let value = value.strip_prefix("#").unwrap();

                if value.len() != 6 {
                    return false;
                }

                if !value.chars().all(|v| ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f'].contains(&v)) {
                    return false;
                }

                true
            }),
            ("ecl", |value: &str| {
                ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&value)
            }),
            ("pid", |value: &str| {
                value.len() == 9 && value.parse::<u32>().is_ok()
            })
        ];

        let mut result = 0;
        for passport in passports.iter() {
            let attributes: Vec<&str> = passport.split_whitespace().collect();
            let mut valid = true;
            for attribute in required_attributes.iter() {
                if !attributes.iter().any(|&a| a.starts_with(attribute.0) && attribute.1(a.split(":").last().unwrap())) {
                    valid = false;
                }
            }

            if valid {
                result += 1;
            }
        }

        result
    }
}