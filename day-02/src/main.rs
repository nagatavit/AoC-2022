use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(PartialEq, Eq, Copy, Clone)]
enum RPS {
    Rock,
    Paper,
    Scissors,
}

fn decipher_rps(input: char) -> RPS {
    match input {
        'A' => RPS::Rock,
        'B' => RPS::Paper,
        'C' => RPS::Scissors,
        'X' => RPS::Rock,
        'Y' => RPS::Paper,
        'Z' => RPS::Scissors,
        _ => RPS::Rock,
    }
}

fn rps_shape_value(input: RPS) -> u32 {
    match input {
        RPS::Rock => 1,
        RPS::Paper => 2,
        RPS::Scissors => 3,
    }
}

fn check_b_score(a: RPS, b: RPS) -> u32 {
    if b == a {
        3
    } else if b == RPS::Rock {
        match a {
            RPS::Paper => 0,
            RPS::Scissors => 6,
            _ => 0,
        }
    } else if b == RPS::Paper {
        match a {
            RPS::Scissors => 0,
            RPS::Rock => 6,
            _ => 0,
        }
    } else {
        // a == RPS::Scissors
        match a {
            RPS::Rock => 0,
            RPS::Paper => 6,
            _ => 0,
        }
    }
}

fn get_winner(a: RPS) -> RPS {
    match a {
        RPS::Rock => RPS::Paper,
        RPS::Paper => RPS::Scissors,
        RPS::Scissors => RPS::Rock,
    }
}

fn get_loser(a: RPS) -> RPS {
    match a {
        RPS::Rock => RPS::Scissors,
        RPS::Paper => RPS::Rock,
        RPS::Scissors => RPS::Paper,
    }
}

fn get_draw(a: RPS) -> RPS {
    a
}

fn get_play(input: char, opponent: RPS) -> RPS {
    match input {
        'X' => get_loser(opponent),
        'Y' => get_draw(opponent),
        'Z' => get_winner(opponent),
        _ => RPS::Rock,
    }
}

fn main() {
    // File hosts must exist in current path before this produces output
    let lines = match read_lines("src/input") {
        Ok(it) => it,
        _ => return,
    };

    let mut partial_score: u32 = 0;

    for line in lines {
        if let Ok(reading) = line {
            let letters: Vec<char> = reading.chars().collect();
            let my_play = get_play(letters[2], decipher_rps(letters[0]));

            partial_score += check_b_score(decipher_rps(letters[0]), my_play);
            partial_score += rps_shape_value(my_play);
        }
    }

    println!("{}", partial_score);
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
