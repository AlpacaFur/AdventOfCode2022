use std::{fs, iter::Peekable, str::Chars, cmp::{max, min}, vec, collections::{HashSet}, fmt::Display};

fn get_number(chars: &mut Peekable<Chars>) -> i32 {
    let mut num: Vec<char> = vec![];
    loop {
        match chars.peek() {
            Some(char) => {
                if char.is_digit(10) || char == &'-' {
                    num.push(chars.next().unwrap())
                } else {
                    break
                }
            },
            None => break
        }
    };

    num.iter().collect::<String>().parse().unwrap()
}

trait Advanceable {
    fn custom_advance_by(&mut self, amount: i32);
}

impl<T: Iterator> Advanceable for T {
    fn custom_advance_by(&mut self, amount: i32) {
        for _ in 0..amount {
            self.next();
        }
    }
}

#[derive(Copy, Clone, Hash, Eq, PartialEq)]
struct Position {
    x: i32,
    y: i32
}

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

struct SensorReading {
    sensor: Position,
    beacon: Position
}

struct UnionRange {
    ranges: Vec<(i32, i32)>
}

fn ranges_overlap(range1: (i32, i32), range2: (i32, i32)) -> bool {
    (range1.0 >= range2.0 && range1.0 <= range2.1) ||
    (range1.1 >= range2.0 && range1.1 <= range2.1) ||
    (range1.0 <= range2.0 && range1.1 >= range2.1)
}

impl UnionRange {
    fn union(&mut self, new_range: (i32, i32)) {
        let mut left_index: Option<usize> = None;
        let mut right_index: Option<usize> = None;
        let mut remove_parts = true;
        for index in 0..(self.ranges.len()) {
            let range = self.ranges[index];
            if left_index.is_none() {
                if ranges_overlap(new_range, range) {
                    left_index = Some(index);
                    right_index = Some(index);
                } else if new_range.1 < range.0 {
                    left_index = Some(index);
                    remove_parts = false;
                    break;
                }
            } else {
                if range.0 > new_range.1 {
                    break;
                } else {
                    right_index = Some(index);
                }
            }
        }
        if left_index.is_none() {
            self.ranges.push(new_range);
        } else if remove_parts {
            let left_index = left_index.unwrap();
            let right_index = right_index.unwrap();
            let super_range = (
                min(self.ranges[left_index].0, new_range.0), 
                max(self.ranges[right_index].1, new_range.1)
            );
            self.ranges.drain(left_index..right_index + 1);
            self.ranges.insert(left_index, super_range);
        } else {
            self.ranges.insert(left_index.unwrap(), new_range);
        }
    }

    #[allow(dead_code)]
    fn print(&self) {
        let printed = self.ranges.iter().map(|range| {
            format!("[{},{}]", range.0, range.1)
        }).collect::<String>();

        println!("{}", printed);
    }

    fn size(&self) -> i32 {
        let mut total = 0;
        for range in &self.ranges {
            total += range.1 - range.0 + 1;
        }
        total
    }

    fn new() -> UnionRange {
        UnionRange {
            ranges: vec![]
        }
    }

    fn try_find_gap(&self, interval: (i32, i32)) -> Option<i32> {
        let mut up_to = interval.0;
        for range in &self.ranges {
            if range.0 <= up_to + 1 {
                up_to = range.1
            } else {
                return Some(up_to + 1)
            }
            if up_to >= interval.1 {
                return None
            }
        };
        return Some(up_to + 1)
    }
}

fn manhattan_distance(pos1: Position, pos2: Position) -> i32 {
    (pos1.x - pos2.x).abs() + (pos1.y - pos2.y).abs()
}

fn get_readings() -> Vec<SensorReading> {
    let input = 
        fs::read_to_string("input")
        .expect("Error reading input file :(");

    input.lines().map(|line| {
        let mut line = line[12..].chars().peekable();
        let sensor_x = get_number(&mut line);
        line.custom_advance_by(4);
        let sensor_y = get_number(&mut line);
        line.custom_advance_by(25);
        let beacon_x = get_number(&mut line);
        line.custom_advance_by(4);
        let beacon_y = get_number(&mut line);

        SensorReading {
            sensor: Position {
                x: sensor_x,
                y: sensor_y
            },
            beacon: Position {
                x: beacon_x,
                y: beacon_y
            },
        }
    }).collect()
}

fn get_ranges(readings: &Vec<SensorReading>, target_line: i32) -> UnionRange {
    let mut union_range = UnionRange::new();

    readings.iter().for_each(|reading| {
        // beacons.insert(reading.beacon);
        let beacon_dist = manhattan_distance(reading.sensor, reading.beacon);
        let target_line_dist = manhattan_distance(reading.sensor, Position { 
            x: reading.sensor.x, 
            y: target_line
        });
        let delta_dist = (beacon_dist - target_line_dist).abs();

        if target_line_dist <= beacon_dist {
            union_range.union((
                reading.sensor.x - delta_dist, 
                reading.sensor.x + delta_dist
            ));
        }
    });

    union_range
}

fn part_1() -> i32 {
    const TARGET_LINE: i32 = 2000000;
    
    let readings = get_readings();
    
    let union_range = get_ranges(&readings, TARGET_LINE);

    let mut beacons: HashSet<Position> = HashSet::new();
    readings.iter().for_each(|reading| {
        beacons.insert(reading.beacon);
    });

    let beacons_in_row = beacons.iter()
        .filter(|pos| pos.y == TARGET_LINE)
        .count();

    union_range.size() - (beacons_in_row as i32)
}

fn part_2() -> i64 {
    let readings = get_readings();
    
    const INTERVAL: (i32, i32) = (0, 4000000);

    for row in INTERVAL.0..INTERVAL.1 {
        let union_range = get_ranges(&readings, row);
        
        match union_range.try_find_gap(INTERVAL) {
            Some(gap) => {
                println!("gap at row: {}, col: {}", row, gap);
                return (gap as i64) * 4000000 + (row as i64);
            },
            None => {}
        }

        
    }

    panic!("No gap found!")

}

fn main() {
    println!("Part 1");
    let part_1 = part_1();
    println!("{part_1}");

    println!("Part 2");
    let part_2 = part_2();
    println!("{part_2}");
}