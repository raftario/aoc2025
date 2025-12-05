use rangemap::RangeInclusiveSet;

use crate::Challenge;

fn split(input: &str) -> (RangeInclusiveSet<i64>, impl Iterator<Item = i64>) {
    let mut lines = input.trim().lines();

    let ranges = lines
        .by_ref()
        .take_while(|line| !line.is_empty())
        .map(|line| {
            let (start, end) = line.split_once('-').unwrap();
            start.parse().unwrap()..=end.parse().unwrap()
        })
        .collect();

    let ids = lines.map(|line| line.parse().unwrap());

    (ranges, ids)
}

pub const PART1: Challenge = Challenge::new(5, 1, |input| {
    let (ranges, ids) = split(input);
    ids.filter(|id| ranges.contains(id)).count() as i64
});

pub const PART2: Challenge = Challenge::new(5, 2, |input| {
    let (ranges, ..) = split(input);
    ranges
        .iter()
        .fold(0, |count, range| count + range.end() - range.start() + 1)
});
