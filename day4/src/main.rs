use std::fs;

fn part_1() -> i32 {
    let total_envelops: i32 = 
        fs::read_to_string("input")
        .expect("Error reading input file :(")
        .trim()
        .to_owned()
        .split("\n")
        .map(|pair| {
            let pairing = string_to_pairing(pair);
            match one_range_envelops_other(pairing.0, pairing.1) {
                true => 1,
                false => 0
            }
        }).sum();

    total_envelops
}

fn string_to_pairing(pair_string: &str) -> ((i32, i32), (i32, i32)) {
    let mut pairs = pair_string.split(",");
    let pair1 = pairs.next().unwrap().split("-").map(|val| val.parse::<i32>().unwrap()).collect::<Vec<i32>>();
    let pair2 = pairs.next().unwrap().split("-").map(|val| val.parse::<i32>().unwrap()).collect::<Vec<i32>>();
    return ((pair1[0], pair1[1]), (pair2[0], pair2[1]))
}

fn one_range_envelops_other(range1: (i32, i32), range2: (i32, i32)) -> bool {
    range1.0 <= range2.0 && range1.1 >= range2.1 ||
    range2.0 <= range1.0 && range2.1 >= range1.1
}  

fn one_range_overlaps_other(range1: (i32, i32), range2: (i32, i32)) -> bool {
    range1.0 >= range2.0 && range1.0 <= range2.1 ||
    range1.1 >= range2.0 && range1.1 <= range2.1 ||
    range2.0 >= range1.0 && range2.0 <= range1.1 ||
    range2.1 >= range1.0 && range2.1 <= range1.1
}  

fn part_2() -> i32 {
    let total_overlaps: i32 = 
        fs::read_to_string("input")
        .expect("Error reading input file :(")
        .trim()
        .to_owned()
        .split("\n")
        .map(|pair| {
            let pairing = string_to_pairing(pair);
            match one_range_overlaps_other(pairing.0, pairing.1) {
                true => 1,
                false => 0
            }
        }).sum();
    
    total_overlaps
}

fn main() {
    println!("Part 1");
    let part_1 = part_1();
    println!("{part_1}");

    println!("Part 2");
    let part_2 = part_2();
    println!("{part_2}");
}