use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

struct Directory {
    name: String,
    id: usize,
    parent: usize,
    total_size: u32,
    sub_directories: HashMap<String, usize>,
    files: HashMap<String, u32>,
}

fn main() {
    // File hosts must exist in current path before this produces output
    let lines = match read_lines("src/input") {
        Ok(it) => it,
        _ => return,
    };

    let mut file_tree: Vec<Directory> = Vec::new();

    let root_dir = Directory {
        name: "/".to_string(),
        parent: 0,
        id: 0,
        total_size: 0,
        sub_directories: HashMap::new(),
        files: HashMap::new(),
    };

    file_tree.push(root_dir);

    let mut current_dir: usize = 0;

    for line in lines {
        if let Ok(reading) = line {
            let line_split = reading.split(' ').collect::<Vec<&str>>();

            // check if this is a command or an output
            if line_split[0] == "$" {
                if line_split[1] == "cd" {
                    if line_split[2] == "/" {
                        current_dir = 0;
                    } else if line_split[2] == ".." {
                        current_dir = file_tree[current_dir].parent;
                    } else {
                        current_dir = *file_tree[current_dir]
                            .sub_directories
                            .get(line_split[2])
                            .unwrap();
                    }
                } else {
                    continue;
                }
            } else if line_split[0] == "dir" {
                if !file_tree[current_dir]
                    .sub_directories
                    .contains_key(line_split[1])
                {
                    let new_id = file_tree.len();

                    let new_sub_dir = Directory {
                        name: line_split[1].to_string(),
                        parent: file_tree[current_dir].id,
                        id: new_id,
                        total_size: 0,
                        sub_directories: HashMap::new(),
                        files: HashMap::new(),
                    };

                    file_tree[current_dir]
                        .sub_directories
                        .insert(line_split[1].to_string(), new_id);
                    file_tree.push(new_sub_dir);
                }
            } else {
                if !file_tree[current_dir].files.contains_key(line_split[1]) {
                    let size = line_split[0].to_string().parse::<u32>().unwrap();
                    let name = line_split[1].to_string();

                    file_tree[current_dir].files.insert(name, size);
                }
            }
        }
    }

    depth_get_sizes(&mut file_tree, 0);

    // depth_print(&file_tree, 0, 0);

    let result = depth_sum(&file_tree, 0);
    println!("part 1: {}", result);

    let total_used_space = file_tree[0].total_size;
    let total_free_space = 70_000_000 - total_used_space;
    let needed_free_space = 30_000_000 - total_free_space;

    println!("{}", needed_free_space);

    let smallest_dir = depth_min_search(&file_tree, 0, needed_free_space, 70_000_000);
    println!("{}", smallest_dir);
}

fn depth_print(file_tree: &Vec<Directory>, current_dir: usize, print_level: u32) {
    for i in 0..print_level {
        print!("  ");
    }
    println!(
        "- {} (dir, size={})",
        file_tree[current_dir].name, file_tree[current_dir].total_size
    );

    for id in file_tree[current_dir].sub_directories.values() {
        depth_print(file_tree, *id, print_level + 1);
    }

    for (file, size) in &file_tree[current_dir].files {
        for i in 0..(print_level + 1) {
            print!("  ");
        }
        println!("- {} (file, size={})", file, size);
    }
}

fn depth_min_search(
    file_tree: &Vec<Directory>,
    current_dir: usize,
    needed_free_space: u32,
    current_smallest_dir: u32,
) -> u32 {
    let mut smallest_dir: u32 = 0;

    if file_tree[current_dir].total_size < needed_free_space {
        return current_smallest_dir;
    } else {
        if file_tree[current_dir].total_size <= current_smallest_dir {
            smallest_dir = file_tree[current_dir].total_size;
        } else {
            smallest_dir = current_smallest_dir;
        }

        for id in file_tree[current_dir].sub_directories.values() {
            let children_smallest_dir =
                depth_min_search(file_tree, *id, needed_free_space, smallest_dir);
            if children_smallest_dir < smallest_dir {
                smallest_dir = children_smallest_dir;
            }
        }
    }

    // if file_tree[current_dir].total_size <= 100000 {
    //     current_sum += file_tree[current_dir].total_size;
    // }

    // for id in file_tree[current_dir].sub_directories.values() {
    //     current_sum += depth_sum(file_tree, *id);
    // }

    smallest_dir
}

fn depth_sum(file_tree: &Vec<Directory>, current_dir: usize) -> u32 {
    let mut current_sum = 0;

    if file_tree[current_dir].total_size <= 100000 {
        current_sum += file_tree[current_dir].total_size;
    }

    for id in file_tree[current_dir].sub_directories.values() {
        current_sum += depth_sum(file_tree, *id);
    }

    current_sum
}

fn depth_get_sizes(file_tree: &mut Vec<Directory>, current_dir: usize) -> u32 {
    let mut total_size: u32 = 0;

    let mut id_list: Vec<usize> = Vec::new();

    for id in file_tree[current_dir].sub_directories.values() {
        id_list.push(*id);
    }

    for id in id_list {
        total_size += depth_get_sizes(file_tree, id);
    }

    for size in file_tree[current_dir].files.values() {
        total_size += size
    }

    file_tree[current_dir].total_size = total_size;

    total_size
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
