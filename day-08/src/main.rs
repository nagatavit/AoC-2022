use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::vec;

fn main() {
    // File hosts must exist in current path before this produces output
    let lines = match read_lines("src/input") {
        Ok(it) => it,
        _ => return,
    };

    let mut tree_map: Vec<Vec<u8>> = Vec::new();

    for line in lines {
        if let Ok(reading) = line {
            let input_row = reading.as_bytes();

            let mut map_row: Vec<u8> = Vec::new();

            for tree in input_row {
                map_row.push(*tree - ('0' as u8));
            }

            tree_map.push(map_row);
        }
    }

    // for row in &tree_map {
    //     println!("{:?}", row);
    // }

    // Four matrix containing the maximum value up until each row or column
    let mut max_map_up_down: Vec<Vec<i32>> = Vec::new();
    let mut max_map_left_right: Vec<Vec<i32>> = Vec::new();
    let mut max_map_down_up: Vec<Vec<i32>> = Vec::new();
    let mut max_map_right_left: Vec<Vec<i32>> = Vec::new();

    let mut last_seen: Vec<[usize; 10]> = Vec::new();
    for _i in 0..tree_map.len() {
        last_seen.push([0; 10]);
    }

    // up->down
    fill_max_matrix(&tree_map, &mut max_map_up_down);
    for row in 0..max_map_up_down.len() {
        for col in 0..max_map_up_down[0].len() {
            let mut min_dist: usize = max_map_up_down.len();
            for search_last_biggest in tree_map[row][col]..10 {
                if (row - last_seen[col][search_last_biggest as usize]) <= min_dist {
                    min_dist = row - last_seen[col][search_last_biggest as usize];
                }
            }
            max_map_up_down[row][col] = min_dist as i32;
            last_seen[col][tree_map[row][col] as usize] = row;
        }
    }

    let mut last_seen: Vec<[usize; 10]> = Vec::new();
    for _i in 0..tree_map[0].len() {
        last_seen.push([0; 10]);
    }

    // left->right
    fill_max_matrix(&tree_map, &mut max_map_left_right);
    for col in 0..max_map_left_right[0].len() {
        for row in 0..max_map_left_right.len() {
            let mut min_dist: usize = max_map_left_right[0].len();
            for search_last_biggest in tree_map[row][col]..10 {
                if (col - last_seen[row][search_last_biggest as usize]) <= min_dist {
                    min_dist = col - last_seen[row][search_last_biggest as usize];
                }
            }
            max_map_left_right[row][col] = min_dist as i32;
            last_seen[row][tree_map[row][col] as usize] = col;
        }
    }

    let mut last_seen: Vec<[usize; 10]> = Vec::new();
    for _i in 0..tree_map.len() {
        last_seen.push([tree_map[0].len() - 1; 10]);
    }

    // down->up
    fill_max_matrix(&tree_map, &mut max_map_down_up);
    for row in (0..(max_map_down_up.len() - 1)).rev() {
        for col in 0..max_map_down_up[0].len() {
            let mut min_dist: usize = max_map_down_up.len();
            for search_last_biggest in tree_map[row][col]..10 {
                if (last_seen[col][search_last_biggest as usize] - row) <= min_dist {
                    min_dist = last_seen[col][search_last_biggest as usize] - row;
                }
            }
            max_map_down_up[row][col] = min_dist as i32;
            last_seen[col][tree_map[row][col] as usize] = row;
        }
    }

    let mut last_seen: Vec<[usize; 10]> = Vec::new();
    for _i in 0..tree_map[0].len() {
        last_seen.push([tree_map.len() - 1; 10]);
    }

    // right->left
    fill_max_matrix(&tree_map, &mut max_map_right_left);
    for col in (0..(max_map_right_left[0].len() - 1)).rev() {
        for row in 0..max_map_right_left.len() {
            let mut min_dist: usize = max_map_right_left[0].len();
            for search_last_biggest in tree_map[row][col]..10 {
                if (last_seen[row][search_last_biggest as usize] - col) <= min_dist {
                    min_dist = last_seen[row][search_last_biggest as usize] - col;
                }
            }
            max_map_right_left[row][col] = min_dist as i32;
            last_seen[row][tree_map[row][col] as usize] = col;
        }
    }

    // println!("up->down");
    // for row in &max_map_up_down {
    //     println!("{:?}", row);
    // }
    // println!("left->right");
    // for row in &max_map_left_right {
    //     println!("{:?}", row);
    // }
    // println!("down->up");
    // for row in &max_map_down_up {
    //     println!("{:?}", row);
    // }
    // println!("right->left");
    // for row in &max_map_right_left {
    //     println!("{:?}", row);
    // }

    let mut best_scenic_score = 0;
    for row in 0..tree_map.len() {
        for col in 0..tree_map[0].len() {
            let current_scenic_score = max_map_up_down[row][col]
                * max_map_left_right[row][col]
                * max_map_down_up[row][col]
                * max_map_right_left[row][col];
            if current_scenic_score > best_scenic_score {
                best_scenic_score = current_scenic_score;
            }
        }
    }

    println!("Best scenic: {}", best_scenic_score);
}

fn fill_max_matrix(tree_map: &Vec<Vec<u8>>, max_map: &mut Vec<Vec<i32>>) {
    *max_map = Vec::new();

    for row in tree_map {
        let max_map_row = vec![0; row.len()];
        max_map.push(max_map_row);
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
