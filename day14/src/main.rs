use core::{panic};
use std::{fs, collections::HashMap, cmp::{max, min}};

enum Cell {
    Sand,
    SandSource,
    Barrier
}

fn place_input_on_grid(input: String, sparse_grid: &mut HashMap<(i32, i32), Cell>) -> ((i32, i32), (i32, i32)) {

    let mut top_left = (500, 0);
    let mut bottom_right = (500, 0);
    input.lines().for_each(|line| {
        let mut coords = line.split(" -> ")
            .map(|coord_pair| {
                let coord_pair = coord_pair.split(",")
                    .map(|str| str.parse::<i32>().unwrap())
                    .collect::<Vec<i32>>();
                match coord_pair[..] {
                    [x, y] => (x, y),
                    _ => panic!()
                }
            });
        let mut prev_coord = coords.next().unwrap();
        
        coords.for_each(|coord| {
            let delta_x = coord.0 - prev_coord.0;
            let delta_y = coord.1 - prev_coord.1;
            for i in 0..max(delta_x.abs(), delta_y.abs()) + 1 {
                let target = (
                    prev_coord.0 + delta_x.signum() * i, 
                    prev_coord.1 + delta_y.signum() * i
                );
                top_left = (
                    target.0.min(top_left.0), 
                    target.1.min(top_left.1)
                );
                bottom_right = (
                    target.0.max(bottom_right.0), 
                    target.1.max(bottom_right.1)
                );
                sparse_grid.insert(target, Cell::Barrier);
            }
            prev_coord = coord;

        })
    });
    (top_left, bottom_right)
}

fn part_1() -> i32 {
    let input = 
        fs::read_to_string("input")
        .expect("Error reading input file :(");
    
    let mut sparse_grid: HashMap<(i32, i32), Cell> = HashMap::new();
    sparse_grid.insert((500, 0), Cell::SandSource);

    let (top_left, bottom_right) = place_input_on_grid(input, &mut sparse_grid);

    let mut count = 0;
    loop {
        let mut sand_grain = (500, 0);
        loop { 
            if sparse_grid.get(&(sand_grain.0, sand_grain.1 + 1)).is_none() {
                sand_grain = (sand_grain.0, sand_grain.1 + 1);
            } else if sparse_grid.get(&(sand_grain.0 - 1, sand_grain.1 + 1)).is_none() {
                sand_grain = (sand_grain.0 - 1, sand_grain.1 + 1);
            } else if sparse_grid.get(&(sand_grain.0 + 1, sand_grain.1 + 1)).is_none() {
                sand_grain = (sand_grain.0 + 1, sand_grain.1 + 1);
            } else {
                break;
            }
            if sand_grain.0 < top_left.0 || 
                sand_grain.0 > bottom_right.0 ||
                sand_grain.1 > bottom_right.1 {
                break;
            }
        }
        if sand_grain.1 > bottom_right.1 {
            break;
        } else {
            sparse_grid.insert(sand_grain, Cell::Sand);
            count += 1;
        }
        
    }

    count
}

#[allow(dead_code)]
fn render_grid(
    sparse_grid: &mut HashMap<(i32, i32), Cell>, 
    top_left: &(i32, i32), 
    bottom_right: &(i32, i32)
) -> String {
    let mut printed: Vec<char> = vec![];

    for y in top_left.1..=bottom_right.1 {
        for x in top_left.0..=bottom_right.0 {
            printed.push(match sparse_grid.get(&(x, y)) {
                Some(Cell::Barrier) => '#',
                Some(Cell::Sand) => 'o',
                Some(Cell::SandSource) => '+',
                None => '.'
            })
        }
        printed.push('\n');
    };

    printed.into_iter().collect::<String>()
}

fn part_2() -> i32 {
    let input = 
        fs::read_to_string("input")
        .expect("Error reading input file :(");
    
    let mut sparse_grid: HashMap<(i32, i32), Cell> = HashMap::new();
    sparse_grid.insert((500, 0), Cell::SandSource);

    let (mut top_left, mut bottom_right) = place_input_on_grid(input, &mut sparse_grid);

    // Make lower bounds one lower than previous void.
    bottom_right = (bottom_right.0, bottom_right.1 + 1);

    let mut count = 0;
    loop {
        let mut sand_grain = (500, 0);
        loop { 
            if sand_grain.1 + 1 > bottom_right.1 {
                break;
            }
            if sparse_grid.get(&(sand_grain.0, sand_grain.1 + 1)).is_none() {
                sand_grain = (sand_grain.0, sand_grain.1 + 1);
            } else if sparse_grid.get(&(sand_grain.0 - 1, sand_grain.1 + 1)).is_none() {
                sand_grain = (sand_grain.0 - 1, sand_grain.1 + 1);
            } else if sparse_grid.get(&(sand_grain.0 + 1, sand_grain.1 + 1)).is_none() {
                sand_grain = (sand_grain.0 + 1, sand_grain.1 + 1);
            } else {
                // Sand is at rest.
                break;
            }
            
        }
        top_left = (min(top_left.0, sand_grain.0), top_left.1);
        bottom_right = (max(bottom_right.0, sand_grain.0), bottom_right.1);

        count += 1;
        if sand_grain.0 == 500 && sand_grain.1 == 0 {
            break;
        } else {
            sparse_grid.insert(sand_grain, Cell::Sand);
        }
        
    }

    // Uncomment to show grid after sand has fallen.
    // println!("{}", render_grid(&mut sparse_grid, &top_left, &bottom_right));
    count
}

fn main() {
    println!("Part 1");
    let part_1 = part_1();
    println!("{part_1}");

    println!("Part 2");
    let part_2 = part_2();
    println!("{part_2}");
}