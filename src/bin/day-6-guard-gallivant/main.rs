use std::{collections::HashMap, fs, process::exit};

fn is_cell_valid(cell: (i32, i32), rows: i32, cols: i32) -> bool {
    cell.0 >= 0 && cell.1 >= 0 && cell.0 < rows && cell.1 < cols
}

fn main() {
    let input = match fs::read_to_string("./src/bin/day-6-guard-gallivant/input.txt") {
        Ok(v) => v,
        Err(e) => {
            println!("Error reading input: {}", e.to_string());
            exit(1);
        }
    };

    let mut input: Vec<Vec<&str>> = input
        .lines()
        .map(|l| l
            .split("")
            .filter(|ch| !ch.is_empty())
            .collect::<Vec<&str>>()
        )
        .collect();

    let rows = input.len();
    let cols = input[0].len();
    
    let mut cell = (0, 0);
    let mut dir = "T";

    let dir_to_step: HashMap<&str, (i32, i32)> = HashMap::from([
        ("T", (-1, 0)),
        ("R", (0, 1)),
        ("B", (1, 0)),
        ("L", (0, -1))
    ]);

    let rotate_dir: HashMap<&str, &str> = HashMap::from([
        ("T", "R"),
        ("R", "B"),
        ("B", "L"),
        ("L", "T")
    ]);

    let blocked_dir_to_step: HashMap<&str, (i32, i32)> = HashMap::from([
        ("T", (1, 1)),
        ("R", (1, -1)),
        ("B", (-1, -1)),
        ("L", (-1, 1))
    ]);

    for i in 0..rows {
        for j in 0..cols {
            if input[i][j] == "^" {
                cell = (i as i32, j as i32);
            }
        }
    }

    let mut count_visited_cells = 0;
    while is_cell_valid(cell, rows.try_into().unwrap(), cols.try_into().unwrap()) {
        let r = cell.0 as usize;
        let c = cell.1 as usize;

        let step = match dir_to_step.get(dir) {
            Some(v) => *v,
            None => {
                println!("Incorrect direction.");
                exit(1);
            }
        };
        
        if input[r][c] == "X" {
            cell = (cell.0 + step.0, cell.1 + step.1);
        } else if input[r][c] == "." || input[r][c] == "^" {
            count_visited_cells += 1;
            input[r][c] = "X";

            cell = (cell.0 + step.0, cell.1 + step.1);
        } else {
            let blocked_step = match blocked_dir_to_step.get(dir) {
                Some(v) => *v,
                None => {
                    println!("Incorrect direction.");
                    exit(1);
                }
            };
            cell = (cell.0 + blocked_step.0, cell.1 + blocked_step.1);
            dir = match rotate_dir.get(dir) {
                Some(v) => *v,
                None => {
                    println!("Incorrect direction to rotate");
                    exit(1);
                }
            }
        }
    }

    println!("Part One answer: {count_visited_cells}");

}