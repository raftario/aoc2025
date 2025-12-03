use aoc2025::CHALLENGES;

fn main() {
    for challenge in CHALLENGES {
        let input = challenge.input();
        let output = challenge.run(&input);
        println!("{challenge}: {output}");
    }
}
