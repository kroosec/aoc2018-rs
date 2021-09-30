use std::cmp::{max, min};
use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;

type Result<T> = ::std::result::Result<T, Box<dyn ::std::error::Error>>;

#[derive(Debug, Clone)]
struct Location {
    x: usize,
    y: usize,
}

#[derive(Debug, Clone)]
struct Square {
    id: usize,
    distance: usize,
    total: usize,
}

type Area = Vec<Vec<Square>>;

fn new_area(x: usize, y: usize) -> Area {
    vec![
        vec![
            Square {
                id: 0,
                distance: usize::MAX,
                total: 0,
            };
            y + 1
        ];
        x + 1
    ]
}

fn get_distance(location: &Location, x: usize, y: usize) -> usize {
    max(location.x, x) - min(location.x, x) + max(location.y, y) - min(location.y, y)
}

fn mark_with_location(area: &mut Area, id: usize, location: &Location) {
    for (x, row) in area.iter_mut().enumerate() {
        for (y, square) in row.iter_mut().enumerate() {
            let distance = get_distance(location, x, y);

            square.total += distance;
            if square.distance > distance {
                square.id = id;
                square.distance = distance;
            } else if square.distance == distance {
                // We mark equal distance between two squares with special value MAX.
                square.id = usize::MAX;
                square.distance = distance;
            };
        }
    }
}

fn get_location(line: &str) -> Result<Location> {
    let mut split = line.split(", ");

    Ok(Location {
        x: split
            .next()
            .ok_or("Location parsing failure")?
            .parse::<usize>()?,
        y: split
            .next()
            .ok_or("Location parsing failure")?
            .parse::<usize>()?
    })
}

fn get_marked_area(input: &str) -> Result<Area> {
    let mut locations = Vec::new();
    let (mut max_x, mut max_y) = (0, 0);
    for line in input.lines() {
        let location = get_location(line)?;

        max_x = max(max_x, location.x);
        max_y = max(max_y, location.y);
        locations.push(location);
    }

    let mut area = new_area(max_x, max_y);
    for (id, location) in locations.iter().enumerate() {
        // Mark the area using each point. 0 for the point, 1 around it, then 2
        mark_with_location(&mut area, id, &location);
    }

    Ok(area)
}

fn part1(area: &Area) -> Result<()> {
    // Count locations, add id's of the ones on the sides of the area to the infinite list that
    // wouldn't count towards the largest finite region.
    let mut area_sizes: HashMap<usize, usize> = HashMap::new();
    let mut infinite = HashSet::new();
    for (x, row) in area.iter().enumerate() {
        for (y, square) in row.iter().enumerate() {
            if x == 0 || y == 0 || x == area.len() - 1 || y == row.len() - 1 {
                infinite.insert(square.id);
            }
            if square.id != usize::MAX {
                *area_sizes.entry(square.id).or_default() += 1;
            }
        }
    }

    let largest = area_sizes
        .iter()
        .filter(|(id, _)| infinite.get(id).is_none())
        .map(|(_, &value)| value)
        .max()
        .ok_or("Largest not found")?;

    assert_eq!(largest, 4060);
    println!("Part1 {}", largest);

    Ok(())
}

fn part2(area: &Area) {
    let count = area
        .iter()
        .map(|row| row.iter().filter(|square| square.total < 10000).count())
        .sum::<usize>();

    assert_eq!(36136, count);
    println!("Part2 {}", count);
}

fn main() -> Result<()> {
    let input = read_to_string("input.txt")?;

    let area = get_marked_area(&input)?;
    part1(&area)?;
    part2(&area);

    Ok(())
}
