use lazy_static::lazy_static;
use regex::Regex;

use std::collections::{BTreeMap, HashMap, HashSet};
use std::fs::read_to_string;

type Result<T> = ::std::result::Result<T, Box<dyn ::std::error::Error>>;

lazy_static! {
    static ref STEP_ORDER: Regex = Regex::new(
        r"^Step (?P<start>\w) must be finished before step (?P<destination>\w) can begin.$"
    )
    .unwrap();
}

// Using a BTreeMap to keep the iteration order.
// As the steps are [A-Z], this could be optimized into an array.
type Dependencies = BTreeMap<char, HashSet<char>>;

fn get_dependencies_graph(input: &str) -> Result<Dependencies> {
    let mut dependencies = Dependencies::new();

    for line in input.lines() {
        let parsed = STEP_ORDER.captures(line).ok_or("Erroneous line ?")?;

        let start = parsed["start"].parse()?;
        dependencies
            .entry(parsed["destination"].parse()?)
            .or_default()
            .insert(start);

        // Make sure all steps exist in the dependencies table.
        dependencies.entry(start).or_default();
    }

    Ok(dependencies)
}

fn part1(dependencies: &Dependencies) -> Result<()> {
    let mut finished_steps = HashSet::new();
    let mut path = String::with_capacity(dependencies.len());

    while path.len() < dependencies.len() {
        for (&step, deps) in dependencies {
            // Step not done yet, and all its dependencies are done
            if finished_steps.get(&step).is_none() && deps.is_subset(&finished_steps) {
                path.push(step);
                finished_steps.insert(step);

                // Break to re-start the looping, and maintain the alphabetical order of steps that
                // can be done at the same time
                break;
            }
        }
    }

    assert_eq!(path, "GKRVWBESYAMZDPTIUCFXQJLHNO");
    println!("Part1 {}", path);
    Ok(())
}

const STEP_MIN_TIME: usize = 60;
const NUM_WORKERS: usize = 5;

fn get_end_time(step: char, start_time: usize) -> usize {
    start_time + (step as usize - 'A' as usize) + STEP_MIN_TIME
}

fn part2(dependencies: &Dependencies) -> Result<()> {
    let mut finished = HashSet::new();
    // Could be a vector of {Step, End_Time}
    let mut in_progress = HashMap::new();

    let mut current_time: usize = 0;
    while finished.len() < dependencies.len() {
        // Clean-up workers that were done by the previous second.
        in_progress.retain(|&step, end_time| {
            if current_time == *end_time + 1 {
                finished.insert(step);
            }

            *end_time >= current_time
        });

        for (&step, deps) in dependencies {
            // Go to the next second if all workers are busy
            if in_progress.len() == NUM_WORKERS {
                break;
            }

            // No in progress or finished, and all dependencies finished
            if in_progress.get(&step).is_none()
                && finished.get(&step).is_none()
                && deps.is_subset(&finished)
            {
                let end_time = get_end_time(step, current_time);
                in_progress.insert(step, end_time);
            }
        }

        current_time += 1;
    }

    assert_eq!(current_time - 1, 903);
    println!("Part2 {:?}", current_time - 1);
    Ok(())
}

fn main() -> Result<()> {
    let input = read_to_string("input.txt")?;

    let dependencies = get_dependencies_graph(&input)?;
    part1(&dependencies)?;
    part2(&dependencies)?;

    Ok(())
}
