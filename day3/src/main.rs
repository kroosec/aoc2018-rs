#[macro_use]
extern crate more_asserts;

use lazy_static::lazy_static;
use regex::Regex;
use std::fs::read_to_string;

type Result<T> = ::std::result::Result<T, Box<dyn ::std::error::Error>>;

type Fabric = [[u32; 1000]; 1000];

fn main() -> Result<()> {
    let input = read_to_string("input.txt")?;

    let marked_fabric = part1(&input);
    part2(&input, &marked_fabric);

    Ok(())
}

struct Claim {
    id: usize,
    x: usize,
    y: usize,
    length: usize,
    height: usize,
}

impl Claim {
    fn new(claim: &str) -> Self {
        lazy_static! {
            static ref CLAIM_REGEX: Regex = Regex::new(
                r"^#(?P<id>\d+) @ (?P<x>\d+),(?P<y>\d+): (?P<length>\d+)x(?P<height>\d+)$"
            )
            .unwrap();
        };

        let parsed = CLAIM_REGEX.captures(claim).unwrap();
        Claim {
            id: parsed["id"].parse().unwrap(),
            x: parsed["x"].parse().unwrap(),
            y: parsed["y"].parse().unwrap(),
            length: parsed["length"].parse().unwrap(),
            height: parsed["height"].parse().unwrap(),
        }
    }
}

fn mark_fabric(fabric: &mut Fabric, line: &str) {
    let claim = Claim::new(line);
    assert_le!(claim.x + claim.length, 1000);
    assert_le!(claim.y + claim.height, 1000);

    for x in (claim.x)..(claim.x + claim.length) {
        for y in (claim.y)..(claim.y + claim.height) {
            fabric[x][y] += 1;
        }
    }
}

fn part1(input: &str) -> [[u32; 1000]; 1000] {
    let mut fabric = [[0u32; 1000]; 1000];
    for line in input.lines() {
        mark_fabric(&mut fabric, line);
    }

    let overlaps = fabric
        .iter()
        .map(|row| row.iter().filter(|&&s| s > 1).count())
        .sum::<usize>();
    assert_eq!(overlaps, 104712);
    println!("Part1: {}", overlaps);

    fabric
}

fn check_claim(fabric: &Fabric, claim: &Claim) -> bool {
    for x in (claim.x)..(claim.x + claim.length) {
        for y in (claim.y)..(claim.y + claim.height) {
            if fabric[x][y] > 1 {
                return false;
            }
        }
    }

    true
}

fn part2(input: &str, fabric: &Fabric) {
    for line in input.lines() {
        let claim = Claim::new(line);
        if check_claim(fabric, &claim) {
            assert_eq!(840, claim.id);
            println!("Part2: {}", claim.id);
        }
    }
}
