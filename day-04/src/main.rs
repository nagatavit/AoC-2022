use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

struct ElfSection {
    lower_bound: u32,
    upper_bound: u32,
}

fn main() {
    // File hosts must exist in current path before this produces output
    let lines = match read_lines("src/input") {
        Ok(it) => it,
        _ => return,
    };

    let mut union_counter = 0;
    let mut intersection_counter = 0;

    for line in lines {
        if let Ok(reading) = line {
            let line_pair = reading.split(',').collect::<Vec<&str>>();

            let left_pair = line_pair[0].split('-').collect::<Vec<&str>>();

            let elf1 = ElfSection {
                lower_bound: left_pair[0].parse::<u32>().unwrap(),
                upper_bound: left_pair[1].parse::<u32>().unwrap(),
            };

            let right_pair = line_pair[1].split('-').collect::<Vec<&str>>();

            let elf2 = ElfSection {
                lower_bound: right_pair[0].parse::<u32>().unwrap(),
                upper_bound: right_pair[1].parse::<u32>().unwrap(),
            };

            if (elf1.lower_bound <= elf2.lower_bound && elf1.upper_bound >= elf2.upper_bound)
                || (elf2.lower_bound <= elf1.lower_bound && elf2.upper_bound >= elf1.upper_bound)
            {
                union_counter += 1;
            }

            if (elf1.upper_bound >= elf2.lower_bound && elf1.lower_bound <= elf2.lower_bound)
                || (elf2.upper_bound >= elf1.lower_bound && elf2.lower_bound <= elf1.lower_bound)
                || (elf1.lower_bound <= elf2.lower_bound && elf1.upper_bound >= elf2.upper_bound)
                || (elf2.lower_bound <= elf1.lower_bound && elf2.upper_bound >= elf1.upper_bound)
            {
                intersection_counter += 1;
            }
        }
    }

    println!("U: {}", union_counter);
    println!("I: {}", intersection_counter);
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
