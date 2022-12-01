use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut current_calories = 0;
    let mut biggest_calories: [i32; 3] = [0; 3];

    // File hosts must exist in current path before this produces output
    let lines = match read_lines("src/input") {
        Ok(it) => it,
        _ => return,
    };

    for line in lines {
        if let Ok(reading) = line {
            if !reading.is_empty() {
                let new_snack = reading.parse::<i32>().unwrap();
                current_calories += new_snack;
            } else {
                current_calories = 0;
            }

            if current_calories > biggest_calories[0] {
                biggest_calories[2] = biggest_calories[1];
                biggest_calories[1] = biggest_calories[0];
                biggest_calories[0] = current_calories;
            }
        }
    }

    println!(
        "1º: {}\n2º: {}\n3º: {}\n",
        biggest_calories[0], biggest_calories[1], biggest_calories[2]
    );

    println!(
        "sum: {}",
        biggest_calories[0] + biggest_calories[1] + biggest_calories[2]
    )
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
