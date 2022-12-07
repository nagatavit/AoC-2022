use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    // File hosts must exist in current path before this produces output
    let lines = match read_lines("src/input") {
        Ok(it) => it,
        _ => return,
    };

    let mut current_char_list: [bool; 52] = [false; 52];
    let mut head_datastream: usize = 0;
    let mut tail_datastream: usize = 0;

    for line in lines {
        if let Ok(reading) = line {
            let data_stream = reading.as_bytes();

            while head_datastream - tail_datastream != 14 {
                let current_char = data_stream[head_datastream] - 97;

                if current_char_list[current_char as usize] {
                    while data_stream[tail_datastream] != data_stream[head_datastream] {
                        current_char_list[(data_stream[tail_datastream] - 97) as usize] = false;
                        tail_datastream += 1;
                    }
                    tail_datastream += 1;
                } else {
                    current_char_list[current_char as usize] = true;
                }

                head_datastream += 1;
            }
        }
    }

    println!("{}", head_datastream);
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
