pub mod part1 {
    use crate::days::input_parser;
    use std::str::FromStr;
    use core::fmt;
    use std::fmt::Formatter;

    #[derive(Copy, Clone)]
    enum Direction {
        North = 0,
        East  = 1,
        South = 2,
        West  = 3
    }

    impl fmt::Display for Direction {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            write!(f, "{}", match self {
                Direction::North => "North",
                Direction::East  => "East",
                Direction::South => "South",
                Direction::West  => "West"
            })
        }
    }

    impl Direction {
        fn from_int(val: i32) -> Direction {
            match val {
                0 => Direction::North,
                1 => Direction::East,
                2 => Direction::South,
                3 => Direction::West,
                _ => {
                    println!("Oops: {}", val);
                    Direction::West
                }
            }
        }
    }

    enum Action {
        North(i32),
        South(i32),
        East(i32),
        West(i32),
        Left(i32),
        Right(i32),
        Forward(i32)
    }

    impl FromStr for Action {
        type Err = String;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let val = s.chars().skip(1).collect::<String>().parse::<i32>().unwrap();
            match s.chars().next().unwrap() {
                'N' => Ok(Action::North(val)),
                'E' => Ok(Action::East(val)),
                'S' => Ok(Action::South(val)),
                'W' => Ok(Action::West(val)),
                'L' => Ok(Action::Left(val)),
                'R' => Ok(Action::Right(val)),
                'F' => Ok(Action::Forward(val)),
                _   => Err(String::from("Unknown command"))
            }
        }
    }

    struct Position {
        x: i32,
        y: i32,
        direction: Direction
    }

    impl fmt::Display for Position {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            write!(f, "Direction: {}, x: {}, y: {}", self.direction, self.x, self.y)
        }
    }

    impl Position {
        fn new() -> Position {
            Position {
                x: 0,
                y: 0,
                direction: Direction::East
            }
        }

        fn do_action(&mut self, action: Action) {
            match action {
                Action::North(val) => self.y += val,
                Action::East(val)  => self.x += val,
                Action::South(val) => self.y -= val,
                Action::West(val)  => self.x -= val,
                Action::Left(degrees) => self.direction = Direction::from_int((self.direction as i32 - degrees / 90).rem_euclid(4)),
                Action::Right(degrees) => self.direction = Direction::from_int((self.direction as i32 + degrees / 90).rem_euclid(4)),
                Action::Forward(val) => match self.direction {
                    Direction::North => self.y += val,
                    Direction::East  => self.x += val,
                    Direction::South => self.y -= val,
                    Direction::West  => self.x -= val
                }
            }
        }
    }

    pub fn start() -> i32 {
        let input = input_parser::parse_file::<Action>("inputs/day12.txt").unwrap();
        let mut pos = Position::new();

        for action in input {
            pos.do_action(action);
            // println!("{}", pos);
        }

        pos.x.abs() + pos.y.abs()
    }
}