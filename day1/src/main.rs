use std::collections::HashSet;
use std::fs::read_to_string;

type Result<T> = ::std::result::Result<T, Box<dyn ::std::error::Error>>;

struct Solution {
    part1: Option<i32>,
    part2: Option<i32>,
}

fn main() -> Result<()> {
    let input = read_to_string("input.txt")?;

    let Solution { part1, part2 } = process_frequencies(&input)?;

    assert_eq!(423, part1.unwrap());
    assert_eq!(61126, part2.unwrap());
    println!("Part1: {}", part1.unwrap());
    println!("Part2: {}", part2.unwrap());
    Ok(())
}

fn process_frequencies(input: &str) -> Result<Solution> {
    let (mut current_frequency, mut part1, mut part2) = (0, None, None);
    let mut reached_steps = HashSet::new();
    reached_steps.insert(current_frequency);

    while part1.is_none() || part2.is_none() {
        for value in input.lines() {
            current_frequency += value.parse::<i32>()?;

            // Find first repeated frequency only
            if part2.is_none() && reached_steps.contains(&current_frequency) {
                part2 = Some(current_frequency);
            } else {
                reached_steps.insert(current_frequency);
            }
        }

        // First reached frequency only
        if part1.is_none() {
            part1 = Some(current_frequency);
        }
    }

    Ok(Solution { part1, part2 })
}
