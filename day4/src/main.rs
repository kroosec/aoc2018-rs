use lazy_static::lazy_static;
use regex::Regex;

use std::collections::HashMap;
use std::fs::read_to_string;

type Result<T> = ::std::result::Result<T, Box<dyn ::std::error::Error>>;

lazy_static! {
    static ref SHIFT_EVENT: Regex = Regex::new(
        r"^\[\d\d\d\d\-\d\d\-\d\d \d\d:(?P<event_time>\d\d)\] (falls asleep|wakes up|Guard #(?P<id>\d+) begins shift)$"
    )
    .unwrap();
}

fn make_sleep_table(input: &str) -> Result<HashMap<usize, [usize; 60]>> {
    let mut lines = input.lines().collect::<Vec<&str>>();
    lines.sort_unstable();

    let mut sleep_table = HashMap::<usize, [usize; 60]>::new();
    let mut current_guard = 0;
    let mut sleep_time: Option<usize> = None;
    for line in lines {
        // Extract the event's time, and the guard's ID if it exists.
        let parsed = SHIFT_EVENT.captures(line).ok_or("Erroneous line ?")?;

        match parsed.name("id") {
            Some(id) => {
                // New shift. Assert that previous guard is not sleeping.
                assert!(sleep_time.is_none());

                current_guard = id.as_str().parse()?;
                sleep_table
                    .entry(current_guard)
                    .or_insert_with(|| [0usize; 60]);
            }
            None => {
                assert!(current_guard > 0);
                match sleep_time {
                    None => {
                        // Fell asleep
                        sleep_time = Some(parsed["event_time"].parse()?);
                    }
                    Some(start_time) => {
                        // Woke-up
                        let sleep_table = sleep_table
                            .get_mut(&current_guard)
                            .ok_or("Wake-up event for inexisting guard ?")?;
                        let end_time = parsed["event_time"].parse()?;
                        for i in start_time..end_time {
                            // Mark the guard's array of sleep
                            sleep_table[i] += 1;
                        }
                        sleep_time = None;
                    }
                }
            }
        }
    }

    Ok(sleep_table)
}

fn part1(sleep_table: &HashMap<usize, [usize; 60]>) -> Result<()> {
    // Find the guard that has the most minutes asleep
    let (best_guard, minutes) = sleep_table
        .iter()
        .max_by_key(|(_, minutes)| minutes.iter().sum::<usize>())
        .ok_or("No sleep guards ?")?;

    // What minute does that guard spend asleep the most?
    let best_minute = minutes
        .iter()
        .enumerate()
        .max_by_key(|(_, times)| *times)
        .map(|(minute, _)| minute)
        .ok_or("Can't find best minute ?")?;

    assert_eq!(101262, best_guard * best_minute);
    println!("Part1 {}", best_guard * best_minute);

    Ok(())
}

fn part2(sleep_table: &HashMap<usize, [usize; 60]>) -> Result<()> {
    // Of all guards, which guard is most frequently asleep on the same minute?
    let (best_guard, minutes) = sleep_table
        .iter()
        .max_by_key(|(_, minutes)| minutes.iter().max())
        .ok_or("Can't find guard with most frequent sleeps ?")?;

    let top_minute = minutes
        .iter()
        .enumerate()
        .max_by_key(|(_, &value)| value)
        .map(|(idx, _)| idx)
        .ok_or("Can't find the top minute ?")?;

    assert_eq!(71976, best_guard * top_minute);
    println!("Part2 {}", best_guard * top_minute);

    Ok(())
}

fn main() -> Result<()> {
    let input = read_to_string("input.txt").expect("Missing input file");

    let sleep_table = make_sleep_table(&input)?;
    part1(&sleep_table)?;
    part2(&sleep_table)?;

    Ok(())
}
