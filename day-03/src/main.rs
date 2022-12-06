use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    // File hosts must exist in current path before this produces output
    let lines = match read_lines("src/input") {
        Ok(it) => it,
        _ => return,
    };

    let mut item_types: [bool; 52] = [false; 52];
    let mut duplicated_already_found: [bool; 52] = [false; 52];
    let mut priority_sum: u32 = 0;

    for line in lines {
        if let Ok(reading) = line {
            let bytes = reading.as_bytes();

            for (i, c) in bytes.iter().enumerate() {
                let mut shifted_pos: u32 = *c as u32;

                if *c as u32 >= 97 {
                    shifted_pos -= 97;
                } else {
                    shifted_pos -= 39;
                }

                if i < reading.len() / 2 {
                    item_types[shifted_pos as usize] = true;
                } else if item_types[shifted_pos as usize]
                    && !duplicated_already_found[shifted_pos as usize]
                {
                    duplicated_already_found[shifted_pos as usize] = true;
                    priority_sum += shifted_pos + 1;
                }
            }
        }

        for elem in item_types.iter_mut() {
            *elem = false;
        }

        for elem in duplicated_already_found.iter_mut() {
            *elem = false;
        }
    }

    println!("{}", priority_sum);
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
