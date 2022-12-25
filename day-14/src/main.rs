use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Coordinate(usize, usize);

// Custom heap because it is currently not possible to change priority
// on an entry in the collections heap
#[derive(Debug, Eq, PartialEq)]
enum TerrainType {
    Rock,
    Sand,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Sand {
    coord: Coordinate,
}

impl Sand {
    fn drop(
        &mut self,
        cave_mapping: &mut HashMap<Coordinate, TerrainType>,
        lowest_point: usize,
        add_floor: bool, // floor for part 2
    ) -> bool {
        loop {
            let mut down = self.coord;
            down.1 += 1;

            let mut down_left = self.coord;
            down_left.0 -= 1;
            down_left.1 += 1;

            let mut down_right = self.coord;
            down_right.0 += 1;
            down_right.1 += 1;

            if !cave_mapping.contains_key(&down) {
                self.coord = down;
            } else if !cave_mapping.contains_key(&down_left) {
                self.coord = down_left;
            } else if !cave_mapping.contains_key(&down_right) {
                self.coord = down_right;
            } else {
                cave_mapping.insert(self.coord, TerrainType::Sand);
                return false;
            }

            if !add_floor {
                if self.coord.1 > lowest_point {
                    return true;
                }
            } else if down.1 == lowest_point + 1 {
                cave_mapping.insert(self.coord, TerrainType::Sand);
                return false;
            }
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

    let mut cave_mapping: HashMap<Coordinate, TerrainType> = HashMap::new();
    let mut top_left: Coordinate = Coordinate(usize::MAX, usize::MAX);
    let mut bottom_right: Coordinate = Coordinate(0, 0);

    for line in lines {
        if let Ok(reading) = line {
            let mut line_segments: Vec<Coordinate> = Vec::new();

            let paths = reading.split("->");
            for path in paths {
                let path = path.trim();

                let coords = path.split(',').collect::<Vec<&str>>();
                let x = coords[0].parse::<usize>().unwrap();
                let y = coords[1].parse::<usize>().unwrap();

                update_map_corners(&Coordinate(x, y), &mut top_left, &mut bottom_right);

                line_segments.push(Coordinate(x, y));
            }

            for (i, coord) in line_segments.iter().enumerate() {
                // skip the first one
                if i == 0 {
                    continue;
                }

                fill_cave_mapping(&mut cave_mapping, &line_segments[i - 1], coord);
            }
        }
    }

    println!("{:?} {:?}", top_left, bottom_right);

    // print_map(&cave_mapping, &top_left, &bottom_right);

    let sand_start_point = Coordinate(500, 0);
    let mut sand_count = 0;

    loop {
        let mut new_sand = Sand {
            coord: sand_start_point,
        };

        if new_sand.drop(&mut cave_mapping, bottom_right.1, false) {
            break;
        }

        sand_count += 1;
    }

    // print_map(&cave_mapping, &top_left, &bottom_right);

    println!("Part 1: {:?}", sand_count);

    // creating a new var here just so I can print the result later
    let lowest_point = bottom_right.1;

    loop {
        let mut new_sand = Sand {
            coord: sand_start_point,
        };

        new_sand.drop(&mut cave_mapping, lowest_point, true);

        update_map_corners(&new_sand.coord, &mut top_left, &mut bottom_right);

        if new_sand.coord == sand_start_point {
            break;
        }

        sand_count += 1;
    }

    // print_map(&cave_mapping, &top_left, &bottom_right);

    // +1 cause one sand gets dropped after the first part
    println!("Part 2: {:?}", sand_count + 1);
}

fn update_map_corners(
    new_coord: &Coordinate,
    top_left: &mut Coordinate,
    bottom_right: &mut Coordinate,
) {
    if new_coord.0 < top_left.0 {
        top_left.0 = new_coord.0;
    }
    if new_coord.0 > bottom_right.0 {
        bottom_right.0 = new_coord.0;
    }
    if new_coord.1 < top_left.1 {
        top_left.1 = new_coord.1;
    }
    if new_coord.1 > bottom_right.1 {
        bottom_right.1 = new_coord.1;
    }
}

fn print_map(
    cave_mapping: &HashMap<Coordinate, TerrainType>,
    top_left: &Coordinate,
    bottom_right: &Coordinate,
) {
    for i in top_left.1..=bottom_right.1 {
        for j in top_left.0..=bottom_right.0 {
            if cave_mapping.contains_key(&Coordinate(j, i)) {
                let value = cave_mapping.get(&Coordinate(j, i)).unwrap();
                if *value == TerrainType::Rock {
                    print!("#");
                } else {
                    print!("o");
                }
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn fill_cave_mapping(
    cave: &mut HashMap<Coordinate, TerrainType>,
    start_coord: &Coordinate,
    end_coord: &Coordinate,
) {
    // Assuming only straight lines (as it was not stated how we should handle diagonal lines)
    if start_coord.0 == end_coord.0 {
        let (start, end) = sort_pair(&start_coord.1, &end_coord.1);

        for i in start..=end {
            cave.insert(Coordinate(start_coord.0, i), TerrainType::Rock);
        }
    } else {
        let (start, end) = sort_pair(&start_coord.0, &end_coord.0);

        for i in start..=end {
            cave.insert(Coordinate(i, start_coord.1), TerrainType::Rock);
        }
    }
}

fn sort_pair(a: &usize, b: &usize) -> (usize, usize) {
    if a < b {
        (*a, *b)
    } else {
        (*b, *a)
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
