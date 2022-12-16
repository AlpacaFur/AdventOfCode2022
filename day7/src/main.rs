use core::{panic};
use std::{fs, vec, collections::HashMap};

enum CDDirs {
    DotDot,
    Root,
    Dir(String)
}

enum TerminalLine {
    CD(CDDirs),
    LS,
    LSFileResult(i32, String),
    LSDirResult(String)
}

#[derive(Debug)]
enum DirectoryContents {
    File(String, i32),
    Directory(String)
}

fn get_input_tokens() -> Vec<TerminalLine> {
    fs::read_to_string("input")
        .expect("Error reading input file :(")
        .lines().map(|line| {
        if line == "$ ls" {
            TerminalLine::LS
        } else if line == "$ cd .." {
            TerminalLine::CD(CDDirs::DotDot)
        } else if line == "$ cd /" {
            TerminalLine::CD(CDDirs::Root)
        } else if line.starts_with("$ cd ") {
            let segments: Vec<&str> = line.split(" ").collect();
            let dir = segments[2].to_owned();
            TerminalLine::CD(CDDirs::Dir(dir))
        } else if line.starts_with("dir ") {
            let segments: Vec<&str> = line.split(" ").collect();
            let dir = segments[1].to_owned();
            TerminalLine::LSDirResult(dir)
        } else {
            let segments: Vec<&str> = line.split(" ").collect();
            let size = segments[0].parse().unwrap();
            let file = segments[1].to_owned();
            TerminalLine::LSFileResult(size, file)
        }
    }).collect()
}

fn populate_directory_contents() -> HashMap::<String, Vec<DirectoryContents>> {
    let mut current_directory: Vec<String> = vec![];
    let tokens = get_input_tokens();
    let mut lines = tokens.iter().peekable();
    let mut directory_contents: HashMap<String, Vec<DirectoryContents>> = HashMap::new();

    loop {
        match lines.next() {
            Some(terminal_line) => {
                match terminal_line {
                    TerminalLine::CD(CDDirs::DotDot) => {
                        current_directory.pop();
                    },
                    TerminalLine::CD(CDDirs::Root) => {
                        current_directory = vec![];
                    },
                    TerminalLine::CD(CDDirs::Dir(dir)) => {
                        current_directory.push(dir.to_string());
                    },
                    TerminalLine::LS => {
                        let current_dir = current_directory.join("/");
                        let mut dir_contents = vec![];
                        loop {
                            match lines.peek() {
                                Some(TerminalLine::LSFileResult(size, name)) => {
                                    lines.next();
                                    dir_contents.push(DirectoryContents::File(name.to_owned(), size.to_owned()))
                                },
                                Some(TerminalLine::LSDirResult(name)) => {
                                    lines.next();
                                    dir_contents.push(DirectoryContents::Directory(name.to_owned()))
                                },
                                _ => break
                            }
                        }
                        directory_contents.insert(current_dir, dir_contents);
                    },
                    TerminalLine::LSFileResult(_, _) => panic!("ls results shouldn't appear without ls's"),
                    TerminalLine::LSDirResult(_) => panic!("ls results shouldn't appear without ls's")
                }
            },
            None => break
        }
    }

    directory_contents
}

fn part_1() -> i32 {
    let directory_contents = populate_directory_contents();

    const UPPER_BOUND: i32 = 100_000;

    let dir_size_total: i32 = directory_contents.keys()
        .filter_map(|dir_name| {
            let size = dir_size(&directory_contents, dir_name);
            if size <= UPPER_BOUND {
                Some(size)
            } else {
                None
            }
        })
        .sum();


    dir_size_total
}

fn dir_size(
    directory_contents: &HashMap<String, Vec<DirectoryContents>>, 
    directory: &str
) -> i32 {
    if directory_contents.get(directory).is_none() {
        println!("{}", directory);
    }
    directory_contents.get(directory).unwrap().iter().map(|entry| {
        match entry {
            DirectoryContents::File(_, size) => size.clone(),
            DirectoryContents::Directory(name) => {
                if directory == "" {
                    dir_size(&directory_contents, name)
                } else {
                    dir_size(&directory_contents, &(directory.to_owned() + "/" + name))
                }
            }
        }
    }).sum()
}


fn part_2() -> i32 {
    let directory_contents = populate_directory_contents();

    const TOTAL_SPACE: i32 = 70_000_000;
    const TARGET_FREE_SPACE: i32 = 30_000_000;
    
    let dir_sizes = directory_contents.keys()
        .map(|dir_name| dir_size(&directory_contents, dir_name));

    let used: i32 = dir_size(&directory_contents, "");
    let space_to_free = TARGET_FREE_SPACE - (TOTAL_SPACE - used);

    dir_sizes.filter(|size| size >= &space_to_free).min().unwrap()
}

fn main() {
    println!("Part 1");
    let part_1 = part_1();
    println!("{part_1}");

    println!("Part 2");
    let part_2 = part_2();
    println!("{part_2}");
}