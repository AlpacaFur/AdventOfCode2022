use std::fs;

const TOP_ELVES: usize = 3;

fn main() {
    println!("Hello, world!");
    let contents = fs::read_to_string("input")
        .expect("Error reading input file :(");
    
    let mut elves: Vec<i32> = vec![];
    let mut current_elf = 0;

    // for line in contents.split("\n") {
    //     if line == "" {
    //         elves.push(current_elf);
    //         current_elf = 0;

    //     } else {
    //         current_elf += line.parse::<i32>().expect("Expected an integer!");
    //     }
    // }

    contents.split("\n").for_each(|line| {
        if line == "" {
            elves.push(current_elf);
            current_elf = 0;

        } else {
            current_elf += line.parse::<i32>().expect("Expected an integer!");
        }
    });

    elves.sort_by(|a, b| b.cmp(a));

    let top_elf = elves[0];
    
    let mut top_3_total = 0;
    for i in 0..TOP_ELVES {
        top_3_total += elves[i];
    }

    println!("Part 1:");
    println!("{top_elf}");

    println!("Part 2:");
    println!("{top_3_total}");

}
