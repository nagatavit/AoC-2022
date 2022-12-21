use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Coordinate(usize, usize);

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct DijkstraState {
    cost: u32,
    current_coord: Coordinate,
    previous_coord: Coordinate,
}

// Custom heap because it is currently not possible to change priority
// on an entry in the collections heap
#[derive(Debug, Eq, PartialEq)]
struct DijkstraHeap {
    entries: Vec<DijkstraState>,
    index_map: HashMap<Coordinate, usize>,
}

impl DijkstraHeap {
    fn new() -> Self {
        DijkstraHeap {
            entries: Vec::new(),
            index_map: HashMap::new(),
        }
    }

    fn left_child(node: usize) -> usize {
        node * 2 + 1
    }

    fn right_child(node: usize) -> usize {
        node * 2 + 2
    }

    fn parent(node: usize) -> usize {
        if node == 0 {
            0
        } else {
            (node - 1) / 2
        }
    }

    fn insert(&mut self, new_entry: DijkstraState) {
        if let std::collections::hash_map::Entry::Vacant(e) =
            self.index_map.entry(new_entry.current_coord)
        {
            let entry_index = self.entries.len();
            self.entries.push(new_entry);
            e.insert(entry_index);
            self.heap_up(entry_index);
        } else {
            let old_entry_index = self.index_map.get(&new_entry.current_coord).unwrap();
            // println!("{:?}: {:?}", old_entry_index, self.entries);
            let old_entry = self.entries[*old_entry_index];

            if new_entry.cost < old_entry.cost {
                self.entries[*old_entry_index] = new_entry;
                self.heap_up(*old_entry_index);
            }
        }
    }

    fn remove(&mut self) -> Option<DijkstraState> {
        if self.entries.is_empty() {
            return None;
        }

        let smallest = self.entries[0];

        self.entries[0] = self.entries[self.entries.len() - 1];

        // Remove smallest and swap the biggest with zero
        self.index_map.insert(self.entries[0].current_coord, 0);
        self.index_map.remove(&smallest.current_coord);

        self.entries.pop();

        self.heap_down(0);

        Some(smallest)
    }

    fn heap_up(&mut self, node: usize) {
        if node > 0 && self.entries[DijkstraHeap::parent(node)].cost > self.entries[node].cost {
            let tmp_node = self.entries[DijkstraHeap::parent(node)];
            self.entries[DijkstraHeap::parent(node)] = self.entries[node];
            self.entries[node] = tmp_node;

            // swap indexes on the hashmap
            self.index_map.insert(
                self.entries[DijkstraHeap::parent(node)].current_coord,
                DijkstraHeap::parent(node),
            );
            self.index_map
                .insert(self.entries[node].current_coord, node);

            self.heap_up(DijkstraHeap::parent(node));
        }
    }

    fn heap_down(&mut self, node: usize) {
        let mut smallest_child_index: usize;

        let left_child_index = DijkstraHeap::left_child(node);
        let right_child_index = DijkstraHeap::right_child(node);

        // check if the left child exists
        if left_child_index < self.entries.len() {
            smallest_child_index = left_child_index;

            // check if the right child exists and if it is smaller
            // than the left
            if right_child_index < self.entries.len()
                && self.entries[right_child_index].cost < self.entries[left_child_index].cost
            {
                smallest_child_index = right_child_index;
            }

            self.entries.swap(smallest_child_index, node);

            // swap indexes on the hashmap
            self.index_map.insert(
                self.entries[smallest_child_index].current_coord,
                smallest_child_index,
            );
            self.index_map
                .insert(self.entries[node].current_coord, node);

            self.heap_down(smallest_child_index);
        }
    }
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

    let mut height_map: Vec<Vec<u32>> = Vec::new();
    let mut start_point: Coordinate = Coordinate(0, 0);
    let mut end_point: Coordinate = Coordinate(0, 0);

    for line in lines {
        if let Ok(reading) = line {
            let input = reading.as_bytes();

            let mut height_line = Vec::new();

            for (j, height) in input.iter().enumerate() {
                let height = match *height as char {
                    'S' => {
                        start_point = Coordinate(height_map.len(), j);
                        0
                    }
                    'E' => {
                        end_point = Coordinate(height_map.len(), j);
                        25
                    }
                    h => (h as u32) - ('a' as u32),
                };
                height_line.push(height);
            }

            height_map.push(height_line);
        }
    }

    println!("Start: {:?}, End: {:?}", start_point, end_point);

    let mut visited_places: HashMap<Coordinate, u32> = HashMap::new();
    let mut frontier = DijkstraHeap::new();

    frontier.insert(DijkstraState {
        cost: 0,
        current_coord: start_point,
        previous_coord: start_point,
    });

    let mut min_path_cost: u32 = 0;

    loop {
        let current_state: DijkstraState;

        if let Some(state) = frontier.remove() {
            current_state = state;
        } else {
            println!("Something went wrong, exiting");
            break;
        }

        let current_coord = current_state.current_coord;
        let current_cost = current_state.cost;
        let current_height =
            height_map[current_state.current_coord.0][current_state.current_coord.1];

        visited_places.insert(current_coord, current_cost);

        if current_coord == end_point {
            min_path_cost = current_cost;
            break;
        }

        let (tmp_x, tmp_y) = (current_coord.0 as isize, current_coord.1 as isize);

        let neighbors: [(isize, isize); 4] = [
            (tmp_x - 1, tmp_y),
            (tmp_x, tmp_y - 1),
            (tmp_x + 1, tmp_y),
            (tmp_x, tmp_y + 1),
        ];

        for (i, j) in neighbors {
            if i < 0 || j < 0 {
                continue;
            }

            let i = i as usize;
            let j = j as usize;

            if i >= height_map.len()
                || j >= height_map[0].len()
                || visited_places.contains_key(&Coordinate(i, j))
                || height_map[i][j] as i32 - current_height as i32 > 1
            {
                continue;
            }

            if !visited_places.contains_key(&Coordinate(i, j))
                || current_cost < *(visited_places.get(&Coordinate(i, j)).unwrap())
            {
                frontier.insert(DijkstraState {
                    cost: visited_places.get(&current_coord).unwrap() + 1,
                    current_coord: Coordinate(i, j),
                    previous_coord: current_coord,
                })
            }
        }
    }

    println!("Minimum cost path: {}", min_path_cost);
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
