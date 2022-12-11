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

    // let row_len = tree_map.len();
    // let col_len = tree_map[0].len();

    // up->down
    fill_max_matrix(&tree_map, &mut max_map_up_down);
    for row in 1..max_map_up_down.len() {
        for col in 0..max_map_up_down[0].len() {
            if tree_map[row - 1][col] as i32 >= max_map_up_down[row - 1][col] {
                max_map_up_down[row][col] = tree_map[row - 1][col] as i32;
            } else {
                max_map_up_down[row][col] = max_map_up_down[row - 1][col];
            }
        }
    }

    // left->right
    fill_max_matrix(&tree_map, &mut max_map_left_right);
    for col in 1..max_map_left_right[0].len() {
        for row in 0..max_map_left_right.len() {
            if tree_map[row][col - 1] as i32 >= max_map_left_right[row][col - 1] {
                max_map_left_right[row][col] = tree_map[row][col - 1] as i32;
            } else {
                max_map_left_right[row][col] = max_map_left_right[row][col - 1];
            }
        }
    }

    // down->up
    fill_max_matrix(&tree_map, &mut max_map_down_up);
    for row in (0..(max_map_down_up.len() - 1)).rev() {
        for col in 0..max_map_down_up[0].len() {
            if tree_map[row + 1][col] as i32 >= max_map_down_up[row + 1][col] {
                max_map_down_up[row][col] = tree_map[row + 1][col] as i32;
            } else {
                max_map_down_up[row][col] = max_map_down_up[row + 1][col];
            }
        }
    }

    // right->left
    fill_max_matrix(&tree_map, &mut max_map_right_left);
    for col in (0..(max_map_right_left[0].len() - 1)).rev() {
        for row in 0..max_map_right_left.len() {
            if tree_map[row][col + 1] as i32 >= max_map_right_left[row][col + 1] {
                max_map_right_left[row][col] = tree_map[row][col + 1] as i32;
            } else {
                max_map_right_left[row][col] = max_map_right_left[row][col + 1];
            }
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

    let mut visible_trees = 0;
    for row in 0..tree_map.len() {
        for col in 0..tree_map[0].len() {
            let current_tree = tree_map[row][col];
            if current_tree as i32 > max_map_up_down[row][col]
                || current_tree as i32 > max_map_left_right[row][col]
                || current_tree as i32 > max_map_down_up[row][col]
                || current_tree as i32 > max_map_right_left[row][col]
            {
                visible_trees += 1
            }
        }
    }

    println!("Visible trees: {}", visible_trees);
}

fn fill_max_matrix(tree_map: &Vec<Vec<u8>>, max_map: &mut Vec<Vec<i32>>) {
    *max_map = Vec::new();

    for row in tree_map {
        let max_map_row = vec![-1; row.len()];
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
