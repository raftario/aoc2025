use crate::Challenge;

fn iter(input: &str) -> impl Iterator<Item = impl ExactSizeIterator<Item = i64> + Clone> {
    input
        .trim()
        .lines()
        .map(|line| line.bytes().map(|b| (b - b'0') as i64))
}

pub const PART1: Challenge = Challenge::new(3, 1, |input| {
    let mut total = 0;

    for line in iter(input) {
        let (index, first) = line
            .clone()
            .enumerate()
            .take(line.len() - 1)
            .max_by(|(li, lj), (ri, rj)| lj.cmp(rj).then(ri.cmp(li)))
            .unwrap();
        let second = line.skip(index + 1).max().unwrap();

        total += 10 * first + second
    }

    total
});

pub const PART2: Challenge = Challenge::new(3, 2, |input| {
    let mut total = 0;

    for line in iter(input) {
        let mut joltage = 0;
        let mut index = usize::MAX;

        for power in (0..12).rev() {
            let (i, j) = line
                .clone()
                .enumerate()
                .take(line.len() - power)
                .skip(index.wrapping_add(1))
                .max_by(|(li, lj), (ri, rj)| lj.cmp(rj).then(ri.cmp(li)))
                .unwrap();

            joltage += 10i64.pow(power as u32) * j;
            index = i;
        }

        total += joltage;
    }

    total
});
