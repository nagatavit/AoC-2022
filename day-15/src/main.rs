use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Coordinate(i32, i32);

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Beacon {
    coord: Coordinate,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Sensor {
    coord: Coordinate,
    manhattan_distance: i32,
    closest_beacon: Beacon,
}

impl Coordinate {
    fn calculate_manhattan_distance(&self, other: &Coordinate) -> i32 {
        let x_dist = self.0 - other.0;
        let y_dist = self.1 - other.1;
        abs_value(x_dist) + abs_value(y_dist)
    }
}

impl Sensor {
    fn calculate_beacon_distance(&mut self) {
        self.manhattan_distance = self
            .coord
            .calculate_manhattan_distance(&self.closest_beacon.coord);
    }
}

fn main() {
    // File hosts must exist in current path before this produces output
    // let lines = match read_lines("src/example-input") {
    let lines = match read_lines("src/input") {
        Ok(it) => it,
        _ => {
            println!("Error reading the input... Exiting");
            return;
        }
    };

    let mut sensors: Vec<Sensor> = Vec::new();

    for line in lines {
        if let Ok(reading) = line {
            let words: Vec<&str> = reading.split(' ').collect();

            let mut new_sensor: Sensor = Sensor {
                coord: Coordinate(0, 0),
                manhattan_distance: 0,
                closest_beacon: Beacon {
                    coord: Coordinate(0, 0),
                },
            };

            new_sensor.coord.0 = words[2][(words[2].find('=').unwrap() + 1)..(words[2].len() - 1)]
                .parse::<i32>()
                .unwrap();
            new_sensor.coord.1 = words[3][(words[3].find('=').unwrap() + 1)..(words[3].len() - 1)]
                .parse::<i32>()
                .unwrap();

            new_sensor.closest_beacon.coord.0 = words[8]
                [(words[8].find('=').unwrap() + 1)..(words[8].len() - 1)]
                .parse::<i32>()
                .unwrap();
            new_sensor.closest_beacon.coord.1 = words[9]
                [(words[9].find('=').unwrap() + 1)..words[9].len()]
                .parse::<i32>()
                .unwrap();

            new_sensor.calculate_beacon_distance();

            // println!("{:?}", new_sensor);
            sensors.push(new_sensor);
        }
    }

    let line_to_search = 2000000;
    let mut positions_without_a_beacon: HashMap<Coordinate, bool> = HashMap::new();

    for sensor in sensors {
        if sensor.closest_beacon.coord.1 == line_to_search {
            positions_without_a_beacon.insert(sensor.closest_beacon.coord, false);
        }

        let y_dist = sensor
            .coord
            .calculate_manhattan_distance(&Coordinate(sensor.coord.0, line_to_search));

        let search_range = sensor.manhattan_distance - y_dist;
        let start_pos = sensor.coord.0 - search_range;
        let end_pos = sensor.coord.0 + search_range;

        for x in start_pos..=end_pos {
            positions_without_a_beacon
                .entry(Coordinate(x, line_to_search))
                .or_insert(true);
        }
    }

    // println!("{:?}", positions_without_a_beacon);

    println!(
        "Positions on line {} without a beacon: {}",
        line_to_search,
        positions_without_a_beacon.values().filter(|v| **v).count()
    );
}

fn abs_value(a: i32) -> i32 {
    if a < 0 {
        -a
    } else {
        a
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
