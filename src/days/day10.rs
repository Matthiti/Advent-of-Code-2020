pub mod part1 {
    use crate::days::input_parser;
    use std::collections::HashMap;

    pub fn start() -> u32 {
        let mut input = input_parser::parse_file::<u32>("inputs/day10.txt").unwrap();
        input.sort();

        let mut differences: HashMap<u32, u32> = HashMap::new();
        let mut last = 0;
        for adapter in input {
            let difference = adapter - last;
            differences.insert(difference, differences.get(&difference).unwrap_or(&0) + 1);
            last = adapter;
        }

        *differences.get(&1).unwrap() * (*differences.get(&3).unwrap() + 1)
    }
}

pub mod part2 {
    use crate::days::input_parser;
    use std::collections::HashMap;

    pub fn start() -> u64 {
        let mut input = input_parser::parse_file::<u64>("inputs/day10.txt").unwrap();

        // Add the start and end adapters
        input.push(0);
        input.push(input.iter().max().unwrap() + 3);

        input.sort();

        distinct_arrangements(input.as_slice(), &mut HashMap::new())
    }

    fn distinct_arrangements<'a>(adapters: &'a[u64], previous_arrangements: &mut HashMap<&'a[u64], u64>) -> u64 {
        if adapters.len() == 1 {
            // We have successfully reached the end
            return 1;
        }

        if previous_arrangements.contains_key(adapters) {
            return *previous_arrangements.get(adapters).unwrap();
        }

        let start = adapters[0];
        let mut total = 0;
        let mut i = 1;
        while i < adapters.len() && adapters[i] - start <= 3 {
            let arr = distinct_arrangements(&adapters[i..], previous_arrangements);
            previous_arrangements.insert(&adapters[i..], arr);

            total += arr;
            i += 1;
        }

        total
    }
}