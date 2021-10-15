use lazy_static::lazy_static;
use regex::Regex;

use std::collections::VecDeque;
use std::fs::read_to_string;

type Result<T> = ::std::result::Result<T, Box<dyn ::std::error::Error>>;

lazy_static! {
    static ref MARBLE_SETTINGS: Regex = Regex::new(
        r"^(?P<num_players>\d+) players; last marble is worth (?P<last_marble>\d+) points\n?$"
    )
    .unwrap();
}

fn parse_input(input: &str) -> Result<(usize, usize)> {
    let parsed = MARBLE_SETTINGS.captures(input).ok_or("Erroneous input ?")?;

    Ok((
        parsed["num_players"].parse()?,
        parsed["last_marble"].parse()?,
    ))
}

fn shift_clockwise(circle: &mut VecDeque<usize>) -> Result<()> {
    let next = circle.pop_front().ok_or("Can't pop-up marble")?;
    circle.push_back(next);

    Ok(())
}

fn shift_counter_clockwise(circle: &mut VecDeque<usize>) -> Result<()> {
    let previous = circle.pop_back().ok_or("Can't pop-up marble")?;
    circle.push_front(previous);

    Ok(())
}

fn play_game(num_players: usize, last_marble: usize) -> Result<usize> {
    let mut circle = VecDeque::with_capacity(last_marble);
    circle.push_front(0);

    let mut scores = vec![0usize; num_players];
    let mut current_player = 0;
    for marble in 1..=last_marble {
        if marble % 23 == 0 {
            // Keep this marble, and take marble 7 positions counter-clockwise
            scores[current_player] += marble;
            for _ in 0..7 {
                shift_counter_clockwise(&mut circle)?;
            }

            scores[current_player] += circle.pop_back().ok_or("Removing inexisting marble")?;
            shift_clockwise(&mut circle)?;
        } else {
            shift_clockwise(&mut circle)?;
            circle.push_back(marble);
        }

        current_player = (current_player + 1) % num_players;
    }

    Ok(*scores.iter().max().ok_or("Can't find highest score")?)
}

fn part1(num_players: usize, last_marble: usize) -> Result<()> {
    let highest_score = play_game(num_players, last_marble)?;

    assert_eq!(highest_score, 370210);
    println!("Part1 {}", highest_score);
    Ok(())
}

fn part2(num_players: usize, last_marble: usize) -> Result<()> {
    let highest_score = play_game(num_players, last_marble)?;

    assert_eq!(highest_score, 3101176548);
    println!("Part2 {}", highest_score);
    Ok(())
}

fn main() -> Result<()> {
    let input = read_to_string("input.txt")?;

    let (num_players, last_marble) = parse_input(&input)?;
    part1(num_players, last_marble)?;
    part2(num_players, last_marble * 100)?;

    Ok(())
}
