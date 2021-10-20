use lazy_static::lazy_static;
use regex::Regex;

use std::collections::HashSet;
use std::fs::read_to_string;

lazy_static! {
    static ref POINT_REGEX: Regex = Regex::new(
        r"^position=< *?(?P<y>-?\d+), *?(?P<x>-?\d+)> velocity=< *?(?P<velocity_y>-?\d+), *?(?P<velocity_x>-?\d+)>"
    )
    .unwrap();
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
    velocity_x: i32,
    velocity_y: i32,
}

struct Board {
    points: Vec<Point>,
    seconds: usize,
}

impl Board {
    fn new() -> Self {
        Self {
            points: Vec::new(),
            seconds: 0,
        }
    }

    fn add(&mut self, x: i32, y: i32, velocity_x: i32, velocity_y: i32) {
        self.points.push(Point {
            x,
            y,
            velocity_x,
            velocity_y,
        });
    }

    fn draw(&self) -> bool {
        let (mut min_x, mut min_y, mut max_x, mut max_y) = (i32::MAX, i32::MAX, i32::MIN, i32::MIN);
        let mut positions = HashSet::new();

        for point in &self.points {
            max_x = max_x.max(point.x);
            max_y = max_y.max(point.y);
            min_x = min_x.min(point.x);
            min_y = min_y.min(point.y);

            positions.insert((point.x, point.y));
        }

        if max_x > 150 || max_y > 300 {
            assert_ne!(10374, self.seconds);
            return false;
        }
        for x in min_x..=max_x {
            for y in min_y..=max_y {
                if positions.get(&(x, y)).is_some() {
                    print!("#");
                } else {
                    print!(" ");
                }
            }
            println!("");
        }

        true
    }
}

type Result<T> = ::std::result::Result<T, Box<dyn ::std::error::Error>>;

fn parse_input(input: &str) -> Result<Board> {
    let mut board = Board::new();
    for line in input.lines() {
        let parsed = POINT_REGEX.captures(line).ok_or("Erroneous input ?")?;

        board.add(
            parsed["x"].parse::<i32>()?,
            parsed["y"].parse::<i32>()?,
            parsed["velocity_x"].parse::<i32>()?,
            parsed["velocity_y"].parse::<i32>()?,
        );
    }

    Ok(board)
}

fn part12(mut board: Board) -> Result<()> {
    for _ in 0..15000 {
        board.seconds += 1;
        for point in board.points.iter_mut() {
            point.x += point.velocity_x;
            point.y += point.velocity_y;
        }

        if board.draw() {
            println!("Part2: {}", board.seconds);
            println!("-----------------------------------------------");
        }
    }

    Ok(())
}

fn main() -> Result<()> {
    let input = read_to_string("input.txt")?;
    let board = parse_input(&input)?;

    // Message: PPNJEENH
    // Seconds: 10375
    part12(board)?;

    Ok(())
}
