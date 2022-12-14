use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Eq, Hash, PartialEq, Clone, Copy, Debug)]
struct Coordinate {
    x: i64,
    y: i64,
}

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
struct Instruction {
    dir: Direction,
    times: u32,
}

fn parse_direction(input: &str) -> Direction {
    match input {
        "U" => Direction::Up,
        "D" => Direction::Down,
        "L" => Direction::Left,
        "R" => Direction::Right,
        _ => Direction::Up,
    }
}

fn main() {
    // File hosts must exist in current path before this produces output
    let lines = match read_lines("src/input") {
        Ok(it) => it,
        _ => return,
    };

    let mut visited_spots: HashMap<Coordinate, bool> = HashMap::new();

    let mut head_movements: Vec<Instruction> = Vec::new();

    for line in lines {
        if let Ok(reading) = line {
            let input = reading.split(' ').collect::<Vec<&str>>();
            let new_instruction = Instruction {
                dir: parse_direction(input[0]),
                times: input[1].parse::<u32>().unwrap(),
            };
            head_movements.push(new_instruction);
        }
    }

    // head will be the index zero
    let mut knots: Vec<Coordinate> = vec![Coordinate { x: 0, y: 0 }; 10];

    for movement in head_movements {
        for _ in 0..movement.times {
            match movement.dir {
                Direction::Up => knots[0].y += 1,
                Direction::Down => knots[0].y -= 1,
                Direction::Left => knots[0].x -= 1,
                Direction::Right => knots[0].x += 1,
            }

            for i in 1..knots.len() {
                if (knots[i - 1].y - knots[i].y).abs() == 2
                    || (knots[i - 1].x - knots[i].x).abs() == 2
                {
                    if knots[i - 1].y == knots[i].y {
                        knots[i].x = (knots[i - 1].x + knots[i].x) / 2;
                    } else if knots[i - 1].x == knots[i].x {
                        knots[i].y = (knots[i - 1].y + knots[i].y) / 2;
                    } else {
                        if knots[i - 1].x > knots[i].x {
                            knots[i].x += 1;
                        } else {
                            knots[i].x -= 1;
                        }

                        if knots[i - 1].y > knots[i].y {
                            knots[i].y += 1;
                        } else {
                            knots[i].y -= 1;
                        }
                    }
                }
            }

            visited_spots.insert(knots[9], true);
        }
    }

    println!("Visited places: {}", visited_spots.len());
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
