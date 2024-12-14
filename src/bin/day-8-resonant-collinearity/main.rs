use std::{collections::HashMap, fs, process::exit};

fn main() {
    let input = match fs::read_to_string("./src/bin/day-8-resonant-collinearity/input.txt") {
        Ok(v) => v,
        Err(_e) => {
            println!("Error parsing input.");
            exit(1);
        }
    };

    let mut input = input
        .lines()
        .map(|l| l
            .split("")
            .filter(|ch| !ch.is_empty())
            .collect::<Vec<&str>>()
        )
        .collect::<Vec<Vec<&str>>>();

    let mut signal_locations: HashMap<&str, Vec<(usize, usize)>> = HashMap::new();

    let rows = input.len();
    let cols = input[0].len();

    for i in 0..rows {
        for j in 0..cols {
            if input[i][j] != "." {
                signal_locations
                    .entry(&input[i][j])
                    .or_insert_with(Vec::new)
                    .push((i, j));
            }
        }
    }

    let mut part_one_answer = 0;

    for key in signal_locations.keys() {
        let locations = signal_locations.get(*key).unwrap();
        for i in 0..locations.len() {
            let l1 = locations[i];
            let x1 = l1.0 as i32;
            let y1 = l1.1 as i32;
            for j in (i + 1)..locations.len() {
                let l2 = locations[j];
                let x2 = l2.0 as i32;
                let y2 = l2.1 as i32;

                let left_x = x1 - (x2 - x1);
                let left_y = y1 - (y2 - y1);

                let right_x = (x2 + (x2 - x1)) as usize;
                let right_y = (y2 + (y2 - y1)) as usize;

                if left_x >= 0 && left_x < rows as i32 && left_y >= 0 && left_y < cols as i32 {
                    input[left_x as usize][left_y as usize] = "#";
                }

                if right_x >= 0 as usize && right_x < rows && right_y >= 0 as usize && right_y < cols {
                    input[right_x][right_y] = "#";
                }
            }
        }
    }

    for i in 0..rows {
        for j in 0..cols {
            if input[i][j] == "#" {
                part_one_answer += 1;
            }
        }
    }

    println!("{part_one_answer}");

    let mut part_two_answer = 0;

    for key in signal_locations.keys() {
        let locations = signal_locations.get(*key).unwrap();
        for i in 0..locations.len() {
            let l1 = locations[i];
            let x1 = l1.0 as i32;
            let y1 = l1.1 as i32;
            for j in (i + 1)..locations.len() {
                let l2 = locations[j];
                let x2 = l2.0 as i32;
                let y2 = l2.1 as i32;

                let mut left_x = x1 - (x2 - x1);
                let mut left_y = y1 - (y2 - y1);

                let mut right_x = x2 + (x2 - x1);
                let mut right_y = y2 + (y2 - y1);

                while left_x >= 0 && left_x < rows as i32 && left_y >= 0 && left_y < cols as i32 {
                    input[left_x as usize][left_y as usize] = "#";
                    left_x -= x2 - x1;
                    left_y -= y2 - y1;
                }

                while right_x >= 0 && right_x < rows as i32 && right_y >= 0 && right_y < cols as i32 {
                    input[right_x as usize][right_y as usize] = "#";
                    right_x += x2 - x1;
                    right_y += y2 - y1;
                }
            }
        }
    }

    for i in 0..rows {
        for j in 0..cols {
            if input[i][j] == "#" {
                part_two_answer += 1;
            }
        }
    }

    println!("{}", part_two_answer);

}