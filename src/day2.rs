use std::ops::RangeInclusive;

use itoa::Buffer;

use crate::Challenge;

fn iter(input: &str) -> impl Iterator<Item = RangeInclusive<i64>> {
    input.trim().split(',').map(|range| {
        let (start, end) = range.split_once('-').unwrap();
        (start.parse().unwrap())..=(end.parse().unwrap())
    })
}

pub const PART1: Challenge = Challenge::new(2, 1, |input| {
    let mut invalid = 0;
    let mut buffer = Buffer::new();

    for id in iter(input).flatten() {
        let s = buffer.format(id);

        if s.len().is_multiple_of(2) {
            let (l, r) = s.split_at(s.len() / 2);
            if l == r {
                invalid += id;
            }
        }
    }

    invalid
});

pub const PART2: Challenge = Challenge::new(2, 2, |input| {
    let mut invalid = 0;

    let mut buffer = Buffer::new();
    'ids: for id in iter(input).flatten() {
        let s = buffer.format(id);

        'factors: for factor in 2..=s.len() {
            if s.len().is_multiple_of(factor) {
                let mut chunks = s.as_bytes().chunks_exact(s.len() / factor);
                let first = chunks.next().unwrap();

                for chunk in chunks {
                    if chunk != first {
                        continue 'factors;
                    }
                }

                invalid += id;
                continue 'ids;
            }
        }
    }

    invalid
});
