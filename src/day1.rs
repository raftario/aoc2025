use crate::Challenge;

fn iter(input: &str) -> impl Iterator<Item = i64> {
    input.trim().lines().map(|line| {
        let (direction, count) = line.split_at(1);
        let count: i64 = count.parse().unwrap();
        match direction {
            "L" => -count,
            "R" => count,
            _ => unreachable!(),
        }
    })
}

pub const PART1: Challenge = Challenge::new(1, 1, |input| {
    let mut zeroes = 0;
    let mut current = 50;

    for rotation in iter(input) {
        current += rotation;
        if current % 100 == 0 {
            zeroes += 1;
        }
    }

    zeroes
});

pub const PART2: Challenge = Challenge::new(1, 2, |input| {
    let mut zeroes = 0;
    let mut current = 50;

    for rotation in iter(input) {
        for _ in 0..rotation.abs() {
            current += rotation.signum();
            if current % 100 == 0 {
                zeroes += 1;
            }
        }
    }

    zeroes
});
