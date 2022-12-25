use std::cmp::Ordering;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::vec;

#[derive(Debug, Eq, PartialEq)]
enum Content {
    List(Vec<Content>),
    Value(u32),
}

#[derive(Debug, Eq, PartialEq)]
enum Result {
    Correct,
    Wrong,
    Continue,
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

    let mut packet_pairs: Vec<(Content, Content)> = Vec::new();

    let mut i = 1;
    for line in lines {
        if i % 3 == 0 {
            i += 1;
            continue;
        }

        if let Ok(reading) = line {
            let mut input_stack: Vec<Content> = Vec::new();
            let input = reading.as_bytes();

            let mut current_num: Option<u32> = None;

            for c in input {
                match *c as char {
                    '[' => input_stack.push(Content::List(Vec::new())),
                    ']' => {
                        if let Some(num) = current_num {
                            let idx = input_stack.len() - 1;
                            if let Content::List(ref mut sub_list) = input_stack[idx] {
                                sub_list.push(Content::Value(num));
                            }
                            current_num = None;
                        }

                        if input_stack.len() > 1 {
                            let sub_content = input_stack.pop().unwrap();
                            let idx = input_stack.len() - 1;
                            if let Content::List(ref mut sub_list) = input_stack[idx] {
                                sub_list.push(sub_content);
                            }
                        };
                    }
                    ',' => {
                        if let Some(num) = current_num {
                            let idx = input_stack.len() - 1;
                            if let Content::List(ref mut sub_list) = input_stack[idx] {
                                sub_list.push(Content::Value(num));
                            }
                            current_num = None;
                        }
                    }
                    // reading one by at the time, we need to construct the int by digit
                    val => {
                        current_num =
                            Some(current_num.unwrap_or(0) * 10 + (val as u32 - '0' as u32))
                    }
                }
            }

            if i % 3 == 1 {
                // right pair is a placeholder for now
                packet_pairs.push((input_stack.pop().unwrap(), Content::Value(0)));
            } else {
                let mut pair = packet_pairs.pop().unwrap();
                pair.1 = input_stack.pop().unwrap();
                packet_pairs.push(pair);
            }

            i += 1;
        }
    }

    let mut index_sum = 0;

    for (i, pair) in packet_pairs.iter().enumerate() {
        // println!("{:?}", pair);

        let is_in_right_order = compare_packet(&pair.0, &pair.1);
        // println!("{:?}", is_in_right_order);

        if is_in_right_order == Result::Correct {
            index_sum += i + 1;
        }
    }

    println!("index sum: {}", index_sum);

    let divider_packet_1 = Content::List(vec![Content::List(vec![Content::Value(2)])]);
    let divider_packet_2 = Content::List(vec![Content::List(vec![Content::Value(6)])]);

    let mut sum_packets_less_than_divider_1 = 0;
    let mut sum_packets_less_than_divider_2 = 0;

    for (i, pair) in packet_pairs.iter().enumerate() {
        let is_less_than_divider = compare_packet(&pair.0, &divider_packet_1);

        if is_less_than_divider == Result::Correct {
            sum_packets_less_than_divider_1 += 1;
        }

        let is_less_than_divider = compare_packet(&pair.1, &divider_packet_1);

        if is_less_than_divider == Result::Correct {
            sum_packets_less_than_divider_1 += 1;
        }

        let is_less_than_divider = compare_packet(&pair.0, &divider_packet_2);

        if is_less_than_divider == Result::Correct {
            sum_packets_less_than_divider_2 += 1;
        }

        let is_less_than_divider = compare_packet(&pair.1, &divider_packet_2);

        if is_less_than_divider == Result::Correct {
            sum_packets_less_than_divider_2 += 1;
        }
    }

    println!("less than div 1: {}", sum_packets_less_than_divider_1);
    println!("less than div 2: {}", sum_packets_less_than_divider_2);

    let div_1_pos = sum_packets_less_than_divider_1 + 1;
    let div_2_pos = sum_packets_less_than_divider_2 + 2;

    println!("decoder key: {}", div_1_pos * div_2_pos);
}

fn compare_packet(left_content_pair: &Content, right_content_pair: &Content) -> Result {
    match (left_content_pair, right_content_pair) {
        (Content::List(left), Content::List(right)) => {
            for (i, (j, right_elements)) in right.iter().enumerate().enumerate() {
                if j + 1 > left.len() {
                    return Result::Correct;
                }

                let sub_comparison = compare_packet(&left[i], right_elements);
                match sub_comparison {
                    Result::Correct => return Result::Correct,
                    Result::Wrong => return Result::Wrong,
                    Result::Continue => continue,
                };
            }

            match left.len().cmp(&right.len()) {
                Ordering::Less => Result::Correct,
                Ordering::Equal => Result::Continue,
                Ordering::Greater => Result::Wrong,
            }
        }
        (Content::List(_), Content::Value(right)) => {
            let right_list = Content::List(vec![Content::Value(*right)]);
            compare_packet(left_content_pair, &right_list)
        }
        (Content::Value(left), Content::List(_)) => {
            let left_list = Content::List(vec![Content::Value(*left)]);
            compare_packet(&left_list, right_content_pair)
        }
        (Content::Value(left), Content::Value(right)) => match left.cmp(right) {
            Ordering::Less => Result::Correct,
            Ordering::Equal => Result::Continue,
            Ordering::Greater => Result::Wrong,
        },
    }
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
