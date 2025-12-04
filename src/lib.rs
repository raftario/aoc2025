use std::{fmt, fs, path::Path};

mod day1;
mod day2;
mod day3;
mod day4;

#[derive(Copy, Clone)]
pub struct Challenge {
    day: usize,
    part: usize,
    implementation: fn(input: &str) -> i64,
}

pub const CHALLENGES: &[Challenge] = &[
    day1::PART1,
    day1::PART2,
    day2::PART1,
    day2::PART2,
    day3::PART1,
    day3::PART2,
    day4::PART1,
    day4::PART2,
];

impl Challenge {
    const fn new(day: usize, part: usize, implementation: fn(input: &str) -> i64) -> Self {
        assert!(1 <= day && day <= 12, "Days are numbered 1 through 12");
        assert!(1 <= part && part <= 2, "There are 2 parts per day");

        Self {
            day,
            part,
            implementation,
        }
    }

    pub const fn day(&self) -> usize {
        self.day
    }

    pub const fn part(&self) -> usize {
        self.part
    }

    pub fn input(&self) -> String {
        fs::read_to_string(Path::new("./inputs").join(self.day.to_string()))
            .unwrap_or_else(|err| panic!("Missing input for {self} ({err})"))
    }

    pub fn run(&self, input: &str) -> i64 {
        (self.implementation)(input)
    }
}

impl fmt::Display for Challenge {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Day {} part {}", self.day, self.part)
    }
}
