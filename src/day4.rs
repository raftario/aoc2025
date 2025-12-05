use std::ops::{Index, IndexMut};

use crate::Challenge;

struct Grid {
    data: Vec<u8>,
    width: usize,
    height: usize,
    newline: usize,
}

impl Grid {
    fn new(input: &str) -> Self {
        let data = input.as_bytes().to_vec();
        let (width, newline) = data
            .iter()
            .enumerate()
            .find(|(.., byte)| **byte == b'\r' || **byte == b'\n')
            .unwrap();
        let newline = if *newline == b'\r' { 2 } else { 1 };
        let height = data.len() / (width + newline);

        Self {
            data,
            width,
            height,
            newline,
        }
    }
}

impl Index<[usize; 2]> for Grid {
    type Output = u8;

    fn index(&self, [x, y]: [usize; 2]) -> &Self::Output {
        let index = x + y * (self.width + self.newline);
        &self.data[index]
    }
}

impl IndexMut<[usize; 2]> for Grid {
    fn index_mut(&mut self, [x, y]: [usize; 2]) -> &mut Self::Output {
        let index = x + y * (self.width + self.newline);
        &mut self.data[index]
    }
}

pub const PART1: Challenge = Challenge::new(4, 1, |input| {
    let grid = Grid::new(input);
    let mut total = 0;

    for y in 0..grid.height {
        for x in 0..grid.width {
            let coords = [x, y];
            if grid[coords] != b'@' {
                continue;
            }

            let mut neighbors = 0;

            for x in x.saturating_sub(1)..=(x + 1) {
                for y in y.saturating_sub(1)..=(y + 1) {
                    let check = [x, y] != coords && x < grid.width && y < grid.height;
                    if check && grid[[x, y]] == b'@' {
                        neighbors += 1;
                    }
                }
            }

            if neighbors < 4 {
                total += 1;
            }
        }
    }

    total
});

pub const PART2: Challenge = Challenge::new(4, 2, |input| {
    let mut grid = Grid::new(input);
    let mut total = 0;

    loop {
        let mut subtotal = 0;

        for y in 0..grid.height {
            for x in 0..grid.width {
                let coords = [x, y];
                if grid[coords] != b'@' {
                    continue;
                }

                let mut neighbors = 0;

                for x in x.saturating_sub(1)..=(x + 1) {
                    for y in y.saturating_sub(1)..=(y + 1) {
                        let check = [x, y] != coords && x < grid.width && y < grid.height;
                        if check && grid[[x, y]] == b'@' {
                            neighbors += 1;
                        }
                    }
                }

                if neighbors < 4 {
                    subtotal += 1;
                    grid[coords] = b'.'
                }
            }
        }

        if subtotal != 0 {
            total += subtotal;
        } else {
            break;
        }
    }

    total
});
