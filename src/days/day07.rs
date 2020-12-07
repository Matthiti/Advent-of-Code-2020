pub mod part1 {
    use crate::days::input_parser;
    use std::collections::{HashMap, HashSet};

    pub fn start() -> u32 {
        let input = input_parser::parse_file::<String>("inputs/day07.txt").unwrap();
        let bags = parse_bags(input);

        let mut lead_to_shiny_gold: HashSet<&String> = HashSet::new();

        for _ in 0..bags.len() {
            for (color, contain_colors) in &bags {
                if lead_to_shiny_gold.contains(color) {
                    continue;
                }

                for contain_color in contain_colors {
                    if lead_to_shiny_gold.contains(contain_color) {
                        lead_to_shiny_gold.insert(color);
                    } else if contain_color == "shiny gold" {
                        lead_to_shiny_gold.insert(color);
                    }
                }
            }
        }

        lead_to_shiny_gold.len() as u32
    }

    fn parse_bags(input: Vec<String>) -> HashMap<String, Vec<String>> {
        let mut bags: HashMap<String, Vec<String>> = HashMap::new();

        for line in &input {
            let parts: Vec<&str> = line.split(" ").collect();
            let color = parts[0..2].join(" ");
            let contains_str = &parts[4..];

            let mut contains = vec![];
            let mut i = 0;
            while i < contains_str.len() {
                contains.push(
                    contains_str[i+1..i+3].join(" ")
                );

                i += 4;
            }

            bags.insert(color, contains);
        }

        bags
    }
}

pub mod part2 {
    use crate::days::input_parser;
    use std::collections::HashMap;

    pub fn start() -> u32 {
        let input = input_parser::parse_file::<String>("inputs/day07.txt").unwrap();
        let bags = parse_bags(input);

        count_bags(&String::from("shiny gold"), &bags) - 1
    }

    fn count_bags(color: &String, bags: &HashMap<String, Vec<(String, u32)>>) -> u32 {
        let mut count = 1;
        let contains = bags.get(color).unwrap();

        for (color, amount) in contains {
            count += amount * count_bags(color, bags);
        }

        count
    }

    fn parse_bags(input: Vec<String>) -> HashMap<String, Vec<(String, u32)>> {
        let mut bags: HashMap<String, Vec<(String, u32)>> = HashMap::new();
        for bag in &input {
            let parts: Vec<&str> = bag.split(" ").collect();
            let color = parts[0..2].join(" ");
            let contains_str = &parts[4..];

            let mut contains: Vec<(String, u32)> = vec![];
            let mut i = 0;
            while i < contains_str.len() {
                if contains_str[i] == "no" {
                    break;
                }

                contains.push((contains_str[i+1..i+3].join(" "), contains_str[i].parse().unwrap()));

                i += 4;
            }

            bags.insert(color, contains);
        }

        bags
    }
}