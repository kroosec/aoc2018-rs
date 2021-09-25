use std::fs::read_to_string;

type Result<T> = ::std::result::Result<T, Box<dyn ::std::error::Error>>;

fn react_polymer(input: &str) -> String {
    let mut polymer = String::with_capacity(input.len());

    for current in input.bytes() {
        if let Some(previous) = polymer.bytes().last() {
            if previous + 32 == current || previous - 32 == current {
                polymer.pop();
                continue;
            }
        }

        polymer.push(current as char);
    }

    polymer
}

fn part1(input: &str) {
    let reacted = react_polymer(input);

    assert_eq!(9238, reacted.len());
    println!("Part1 {}", reacted.len());
}

fn part2(input: &str) {
    let lowest_count = ('a'..='z')
        .map(|c| {
            let filtered = input
                .chars()
                .filter(|ch| ch.to_ascii_lowercase() != c)
                .collect::<String>();

            let reacted = react_polymer(&filtered);

            reacted.len()
        })
        .min()
        .unwrap();

    assert_eq!(4052, lowest_count);
    println!("Part2 {}", lowest_count);
}

fn main() -> Result<()> {
    let input = read_to_string("input.txt")?;

    part1(&input);
    part2(&input);

    Ok(())
}
