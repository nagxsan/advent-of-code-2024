use std::{fs, process::exit};

fn part_one_answer(input: &Vec<&str>) {
    let mut blocks: Vec<String> = Vec::new();

    for i in 0..input.len() {
        let num_blocks = input[i].parse::<u8>().unwrap();
        let temp: String;
        if i % 2 == 0 {
            temp = (i / 2).to_string();
        } else {
            temp = String::from(".");
        }
        for _ in 0..num_blocks {
            blocks.push(temp.clone());
        }
    }

    let mut i: usize = 0;
    for idx in 0..blocks.len() {
        if blocks[idx] == "." {
            i = idx;
            break;
        }
    }

    while blocks.len() > i {
        let last_el = blocks.pop().unwrap();
        if last_el != "." {
            blocks[i] = last_el;
            for idx in (i + 1)..blocks.len() {
                if blocks[idx] == "." {
                    i = idx;
                    break;
                }
            }
            if blocks[i] != "." {
                break;
            }
        }
    }

    let mut part_one_answer = 0;
    for i in 0..blocks.len() {
        part_one_answer += i as i128 * blocks[i].parse::<i128>().unwrap();
    }

    println!("{part_one_answer}");
}

fn part_two_answer(input: &Vec<&str>) {
    let mut blocks: Vec<String> = Vec::new();

    for i in 0..input.len() {
        let num_blocks = input[i].parse::<u8>().unwrap();
        let temp: String;
        if i % 2 == 0 {
            temp = (i / 2).to_string();
        } else {
            temp = String::from(".");
        }
        for _ in 0..num_blocks {
            blocks.push(temp.clone());
        }
    }

    let mut part_two_answer = 0;


    

    println!("{part_two_answer}");
}

fn main() {
    let input = match fs::read_to_string("./src/bin/day-9-disk-fragmenter/input.txt") {
        Ok(v) => v,
        Err(_e) => {
            println!("Error parsing input.");
            exit(1);
        }
    };

    let input = input
        .split("")
        .filter(|ch| !ch.is_empty())
        .collect::<Vec<&str>>();

    part_one_answer(&input);
    part_two_answer(&input);
}