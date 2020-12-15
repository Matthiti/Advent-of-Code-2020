use std::str::FromStr;

#[derive(Copy, Clone)]
enum Mask {
    One,
    Zero,
    X
}

impl FromStr for Mask {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "1" => Ok(Self::One),
            "0" => Ok(Self::Zero),
            "X" => Ok(Self::X),
            _   => Err(String::from("Unknown mask"))
        }
    }
}

enum Instruction {
    SetMask([Mask; 36]),
    WriteMemory(u64, u64)
}

impl FromStr for Instruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with("mask") {
            let mask_str = s.split("=").collect::<Vec<&str>>()[1].trim();
            let mut mask = [Mask::X; 36];
            let mut i = 0;

            for c in mask_str.chars() {
                mask[mask.len() - 1 - i] = c.to_string().parse().unwrap();
                i += 1;
            }

            Ok(Instruction::SetMask(mask))
        } else {
            let parts: Vec<&str> = s.split("=").collect();
            let address = parts[0].strip_prefix("mem[").unwrap().trim().strip_suffix("]").unwrap().parse::<u64>().unwrap();
            let value = parts[1].trim().parse::<u64>().unwrap();
            Ok(Instruction::WriteMemory(address, value))
        }
    }
}

pub mod part1 {
    use crate::days::input_parser;
    use std::collections::HashMap;
    use crate::days::day14::{Instruction, Mask};

    struct Program {
        memory: HashMap<u64, u64>,
        instructions: Vec<Instruction>,
        mask: [Mask; 36]
    }

    impl Program {
        pub fn new(instructions: Vec<Instruction>) -> Program {
            Program {
                memory: HashMap::new(),
                instructions,
                mask: [Mask::X; 36]
            }
        }

        fn run(&mut self) {
            for instruction in &self.instructions {
                match instruction {
                    Instruction::SetMask(mask) => {
                        self.mask = *mask;
                    },
                    Instruction::WriteMemory(address, value) => {
                        let mut mask_val = 0;
                        let mut i = 0;
                        for m in &self.mask {
                            mask_val += match m {
                                Mask::One => {
                                    2u64.pow(i as u32)
                                },
                                Mask::Zero => {
                                    0
                                },
                                Mask::X => {
                                    Self::get_bit_at(*value, i)
                                }
                            };
                            i += 1;
                        }

                        self.memory.insert(*address, mask_val);
                    }
                }
            }
        }

        fn get_bit_at(value: u64, n: u64) -> u64 {
            value & (1 << n)
        }
    }

    pub fn start() -> u64 {
        let input = input_parser::parse_file::<Instruction>("inputs/day14.txt").unwrap();
        let mut program = Program::new(input);

        program.run();

        let mut result = 0;
        for (_, val) in program.memory.iter() {
            result += *val;
        }

        result
    }
}

pub mod part2 {
    use crate::days::input_parser;
    use std::collections::HashMap;
    use crate::days::day14::{Instruction, Mask};

    struct Program {
        memory: HashMap<u64, u64>,
        instructions: Vec<Instruction>,
        mask: [Mask; 36]
    }

    impl Program {
        pub fn new(instructions: Vec<Instruction>) -> Program {
            Program {
                memory: HashMap::new(),
                instructions,
                mask: [Mask::X; 36]
            }
        }

        fn run(&mut self) {
            let mut j = 0;
            for instruction in &self.instructions {
                println!("{}", j);
                j += 1;
                match instruction {
                    Instruction::SetMask(mask) => {
                        self.mask = *mask;
                    },
                    Instruction::WriteMemory(address, value) => {
                        let mut mask_address = 0;
                        let mut floating = vec![];
                        let mut i = 0;
                        for m in &self.mask {
                            mask_address += match m {
                                Mask::One => {
                                    2u64.pow(i as u32)
                                },
                                Mask::Zero => {
                                    Self::get_bit_at(*address, i)
                                },
                                Mask::X => {
                                    floating.push(i);
                                    0
                                }
                            };
                            i += 1;
                        }

                        for address in Self::get_all_addresses(mask_address, floating) {
                            self.memory.insert(address, *value);
                        }
                    }
                }
            }
        }

        fn get_bit_at(value: u64, n: u64) -> u64 {
            value & (1 << n)
        }

        fn get_all_addresses(address: u64, floating: Vec<u64>) -> Vec<u64> {
            if floating.len() == 0 {
                return vec![address];
            }

            let mut result = vec![];
            for i in &floating {
                result.append(
                    &mut Self::get_all_addresses(address | 2u64.pow(*i as u32), floating[1..].to_vec())
                );
                result.append(
                    &mut Self::get_all_addresses(address & !(1 << i), floating[1..].to_vec())
                );
            }
            result
        }
    }

    pub fn start() -> u64 {
        let input = input_parser::parse_file::<Instruction>("inputs/day14.txt").unwrap();
        let mut program = Program::new(input);

        program.run();

        let mut result = 0;
        for (_, val) in program.memory.iter() {
            result += *val;
        }

        result
    }
}