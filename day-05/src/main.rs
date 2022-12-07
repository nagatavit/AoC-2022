use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    // File hosts must exist in current path before this produces output
    let lines = match read_lines("src/input") {
        Ok(it) => it,
        _ => return,
    };

    let mut pilar_list: Vec<Vec<char>> = Vec::new();
    let mut is_instruction_lines = false;

    for line in lines {
        if let Ok(reading) = line {
            if reading.is_empty() {
                // already read the initial board, now continue to the rest
                for pilar in pilar_list.iter_mut() {
                    pilar.reverse();
                }
                is_instruction_lines = true;
                continue;
            }

            if !is_instruction_lines {
                for i in (1..reading.len() - 1).step_by(4) {
                    let pilar_num = (i - 1) / 4;

                    if pilar_num + 1 > pilar_list.len() {
                        let pilar: Vec<char> = Vec::new();
                        pilar_list.push(pilar);
                    }

                    let current_crate = reading.chars().nth(i).unwrap();
                    if current_crate != ' ' {
                        pilar_list[pilar_num].push(current_crate);
                    }
                }
            } else {
                let instructions = reading.split(' ').collect::<Vec<&str>>();

                let iterations: u32 = instructions[1].parse::<u32>().unwrap();
                let origin: u32 = instructions[3].parse::<u32>().unwrap() - 1;
                let destiny: u32 = instructions[5].parse::<u32>().unwrap() - 1;

                // part 1
                // for _i in 0..iterations {
                //     let object_crate = pilar_list[origin as usize].pop().unwrap();
                //     pilar_list[destiny as usize].push(object_crate);
                // }

                // part 2
                let mut tmp_crate_holder: Vec<char> = Vec::new();
                for _i in 0..iterations {
                    let object_crate = pilar_list[origin as usize].pop().unwrap();
                    tmp_crate_holder.push(object_crate);
                }

                for _i in 0..iterations {
                    let object_crate = tmp_crate_holder.pop().unwrap();
                    pilar_list[destiny as usize].push(object_crate);
                }
            }
        }
    }

    for pilar in pilar_list.iter_mut() {
        print!("{}", pilar.pop().unwrap());
    }

    println!("");
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
