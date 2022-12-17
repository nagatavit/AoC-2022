use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug)]
struct Monkey {
    items: Vec<u32>,
    id: u32,
    op: Operation,
    operands: (String, String),
    divisible_by_test: u32,
    true_condition_monkey: u32,
    false_condition_monkey: u32,
}

impl Monkey {
    fn inspect(&mut self) {
        for item in &mut self.items {
            let mut op1 = 0;

            if self.operands.0 == "old" {
                op1 = *item
            } else {
                op1 = self.operands.1.parse::<u32>().unwrap();
            }

            let mut op2 = 0;

            if self.operands.1 == "old" {
                op2 = *item
            } else {
                op2 = self.operands.1.parse::<u32>().unwrap();
            }

            *item = match self.op {
                Operation::Sum => op1 + op2,
                Operation::Mul => op1 * op2,
                Operation::None => 0,
            }
        }
    }

    fn reduce_worry(&mut self) {
        for item in &mut self.items {
            *item /= 3;
        }
    }

    fn send_to_monkey(&self, item: u32) -> u32 {
        if item % self.divisible_by_test == 0 {
            self.true_condition_monkey
        } else {
            self.false_condition_monkey
        }
    }
}

#[derive(Debug)]
enum Operation {
    Sum,
    Mul,
    None,
}

fn main() {
    // File hosts must exist in current path before this produces output
    let lines = match read_lines("src/input") {
        Ok(it) => it,
        _ => {
            println!("Error reading the input... Exiting");
            return;
        }
    };

    let mut monkeys: Vec<Monkey> = Vec::new();

    for line in lines {
        if let Ok(reading) = line {
            let reading = reading.trim();
            let input = reading.split(' ').collect::<Vec<&str>>();

            let mut monkey_pos = 0;
            if !monkeys.is_empty() {
                monkey_pos = monkeys.len() - 1;
            }

            if input[0] == "Monkey" {
                let id_len = input[1].len() - 1;
                let id = (&input[1][0..id_len]).parse::<u32>().unwrap();

                let new_monkey = Monkey {
                    items: Vec::new(),
                    id,
                    op: Operation::None,
                    operands: ("0".to_string(), "0".to_string()),
                    divisible_by_test: 0,
                    true_condition_monkey: 0,
                    false_condition_monkey: 0,
                };
                monkeys.push(new_monkey);
            } else if input[0] == "Starting" {
                let mut items = Vec::new();
                for item in &input[2..input.len()] {
                    let item = item.replace(',', "").parse::<u32>().unwrap();
                    items.push(item);
                }
                monkeys[monkey_pos].items = items;
            } else if input[0] == "Operation:" {
                if input[4] == "+" {
                    monkeys[monkey_pos].op = Operation::Sum;
                } else {
                    monkeys[monkey_pos].op = Operation::Mul;
                }

                monkeys[monkey_pos].operands = (input[3].to_string(), input[5].to_string());
            } else if input[0] == "Test:" {
                monkeys[monkey_pos].divisible_by_test = input[3].parse::<u32>().unwrap();
            } else if input[0] == "If" {
                let dst_monkey = input[5].parse::<u32>().unwrap();

                if input[1] == "true:" {
                    monkeys[monkey_pos].true_condition_monkey = dst_monkey;
                } else {
                    monkeys[monkey_pos].false_condition_monkey = dst_monkey;
                }
            }
        }
    }

    let mut inspect_count: Vec<u32> = vec![0; monkeys.len()];

    // Rounds
    for _id in 0..20 {
        // println!("================");
        // println!("Round: {:?}:", id);
        // for monkey in &monkeys {
        //     println!("   Monkey {:?}: {:?}", monkey.id, monkey.items);
        // }

        for monkey_idx in 0..monkeys.len() {
            inspect_count[monkey_idx] += monkeys[monkey_idx].items.len() as u32;

            monkeys[monkey_idx].inspect();
            monkeys[monkey_idx].reduce_worry();

            let items = monkeys[monkey_idx].items.clone();
            for item in items {
                let next_monkey = monkeys[monkey_idx].send_to_monkey(item);
                monkeys[next_monkey as usize].items.push(item);
            }
            monkeys[monkey_idx].items = Vec::new();
        }
    }

    println!("inspect counts: {:?}", inspect_count);

    inspect_count.sort();

    println!(
        "monkey business: {}",
        inspect_count[inspect_count.len() - 1] * inspect_count[inspect_count.len() - 2]
    );
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
