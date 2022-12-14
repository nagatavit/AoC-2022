use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    // File hosts must exist in current path before this produces output
    let lines = match read_lines("src/input") {
        Ok(it) => it,
        _ => return,
    };

    let important_cycles: Vec<i32> = vec![20, 60, 100, 140, 180, 220];
    let mut current_cycle: i32 = 0;
    let mut register_x: i32 = 1;
    let mut sig_strength_sum: i32 = 0;

    for line in lines {
        if let Ok(reading) = line {
            let input = reading.split(' ').collect::<Vec<&str>>();

            if input[0] == "addx" {
                current_cycle += 1;
                if important_cycles.contains(&current_cycle) {
                    sig_strength_sum += current_cycle * register_x;
                }
                current_cycle += 1;
                if important_cycles.contains(&current_cycle) {
                    sig_strength_sum += current_cycle * register_x;
                }
                register_x += input[1].parse::<i32>().unwrap();
            } else {
                current_cycle += 1;
                if important_cycles.contains(&current_cycle) {
                    sig_strength_sum += current_cycle * register_x;
                }
            }

            // println!("{}: {}", current_cycle, register_x);
        }
    }
    println!("{}", sig_strength_sum);
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
