pub mod part1 {
    use std::str::{FromStr, Split};
    use std::num::ParseIntError;
    use std::error::Error;
    use core::fmt;
    use std::fmt::Formatter;
    use crate::days::input_parser;

    enum Instruction {
        Acc(i32),
        Jmp(i32),
        Nop
    }

    #[derive(Debug)]
    enum InstructionError {
        InvalidArgumentError,
        UnknownInstructionError
    }

    impl fmt::Display for InstructionError {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            write!(f, "{}", match self {
                InstructionError::InvalidArgumentError => "Invalid argument",
                InstructionError::UnknownInstructionError => "Unknown instruction"
            })
        }
    }

    impl Error for InstructionError {}

    impl FromStr for Instruction {
        type Err = InstructionError;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let parts: Vec<&str> = s.split(" ").collect();

            match (parts.len(), parts[0]) {
                (2, "acc") => {
                    let arg = parts[1].parse();
                    if arg.is_err() {
                        return Err(InstructionError::InvalidArgumentError)
                    }
                    Ok(Instruction::Acc(arg.unwrap()))
                },
                (2, "jmp") => {
                    let arg = parts[1].parse();
                    if arg.is_err() {
                        return Err(InstructionError::InvalidArgumentError)
                    }
                    Ok(Instruction::Jmp(arg.unwrap()))
                },
                (2, "nop") => Ok(Instruction::Nop),
                _ => Err(InstructionError::UnknownInstructionError)
            }
        }
    }

    pub fn start() -> i32 {
        let instructions = input_parser::parse_file::<Instruction>("inputs/day08.txt").unwrap();

        let mut visited = vec![];
        let mut acc = 0;
        let mut current = 0;

        while !visited.contains(&current) {
            visited.push(current);
            match &instructions[current] {
                Instruction::Acc(val) => {
                    acc += val;
                    current += 1;
                },
                Instruction::Jmp(val) => {
                    if val.is_positive() {
                        current += *val as usize
                    } else {
                        current -= val.abs() as usize
                    }
                },
                Instruction::Nop => current += 1
            }
        }

        acc
    }
}

pub mod part2 {
    use core::fmt;
    use std::fmt::Formatter;
    use std::error::Error;
    use std::str::FromStr;
    use crate::days::input_parser;

    #[derive(Clone)]
    enum Instruction {
        Acc(i32),
        Jmp(i32),
        Nop(i32)
    }

    #[derive(Debug)]
    enum InstructionError {
        InvalidArgumentError,
        UnknownInstructionError
    }

    impl fmt::Display for InstructionError {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            write!(f, "{}", match self {
                InstructionError::InvalidArgumentError => "Invalid argument",
                InstructionError::UnknownInstructionError => "Unknown instruction"
            })
        }
    }

    impl Error for InstructionError {}

    impl FromStr for Instruction {
        type Err = InstructionError;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let parts: Vec<&str> = s.split(" ").collect();

            if parts.len() != 2 {
                return Err(InstructionError::UnknownInstructionError);
            }

            let arg = parts[1].parse();
            if arg.is_err() {
                return Err(InstructionError::InvalidArgumentError)
            }

            match parts[0] {
                "acc" => Ok(Instruction::Acc(arg.unwrap())),
                "jmp" => Ok(Instruction::Jmp(arg.unwrap())),
                "nop" => Ok(Instruction::Nop(arg.unwrap())),
                _ => Err(InstructionError::UnknownInstructionError)
            }
        }
    }

    pub fn start() -> i32 {
        let instructions = input_parser::parse_file::<Instruction>("inputs/day08.txt").unwrap();

        let mut changed_last = 0;
        loop {
            let mut changed_instructions = instructions.clone();
            changed_last = find_next_changeable_instruction(&instructions, changed_last + 1);

            changed_instructions[changed_last] = match &instructions[changed_last] {
                Instruction::Nop(val) => Instruction::Jmp(*val),
                Instruction::Jmp(val) => Instruction::Nop(*val),
                Instruction::Acc(val) => Instruction::Acc(*val) // Should never happen
            };

            let (terminates, acc) = terminates(&changed_instructions);
            if terminates {
                return acc;
            }
        }
    }

    fn find_next_changeable_instruction(instructions: &Vec<Instruction>, mut index: usize) -> usize {
        loop {
            match instructions[index] {
                Instruction::Nop(_) | Instruction::Jmp(_) => return index,
                _ => index += 1
            }
        }
    }

    fn terminates(instructions: &Vec<Instruction>) -> (bool, i32) {
        let mut visited = vec![];
        let mut acc = 0;
        let mut current = 0;

        while current < instructions.len() && !visited.contains(&current) {
            visited.push(current);
            match &instructions[current] {
                Instruction::Acc(val) => {
                    acc += val;
                    current += 1;
                },
                Instruction::Jmp(val) => {
                    if val.is_positive() {
                        current += *val as usize
                    } else {
                        current -= val.abs() as usize
                    }
                },
                Instruction::Nop(_) => current += 1
            }
        }

        if current == instructions.len() {
            (true, acc)
        } else {
            (false, acc)
        }
    }
}