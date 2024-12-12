use std::{collections::{HashMap, HashSet}, fs, process::exit};

fn is_cell_valid(cell: (i32, i32), rows: i32, cols: i32) -> bool {
    cell.0 >= 0 && cell.1 >= 0 && cell.0 < rows && cell.1 < cols
}

fn can_add_obstruction(
    input: &Vec<Vec<&str>>,
    r: usize,
    c: usize,
    dir: &str,
    visited_obstructions: &HashSet<(i32, i32, &str)>,
    dir_to_step: &HashMap<&str, (i32, i32)>,
    rows: i32,
    cols: i32
) -> bool {
    let step = *dir_to_step.get(dir).unwrap();
    let mut cell: (i32, i32) = (r.try_into().unwrap(), c.try_into().unwrap());
    while is_cell_valid(cell, rows, cols) {
        if visited_obstructions.contains(&(cell.0, cell.1, dir)) {
            return true;
        }
        cell = (cell.0 + step.0, cell.1 + step.1);
    }
    false
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

    let mut starting_cell = (0, 0);

    for i in 0..rows {
        for j in 0..cols {
            if input[i][j] == "^" {
                cell = (i as i32, j as i32);
                starting_cell = (i as i32, j as i32);
            }
        }
    }

    let mut visited_obstructions: HashSet<(i32, i32, &str)> = HashSet::new();

    let mut count_visited_cells = 0;
    let mut new_obstructions: HashSet<(i32, i32)> = HashSet::new();
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

        visited_obstructions.insert((cell.0, cell.1, dir));
        
        if input[r][c] == "X" {
            if can_add_obstruction(
                &input,
                r,
                c,
                rotate_dir.get(&dir).unwrap(),
                &visited_obstructions,
                &dir_to_step,
                rows.try_into().unwrap(),
                cols.try_into().unwrap()
            ) {
                let next_step = *dir_to_step.get(dir).unwrap();
                let potential = (cell.0 + next_step.0, cell.1 + next_step.1);
                if potential != starting_cell {
                    new_obstructions.insert(potential);
                }
            }
            cell = (cell.0 + step.0, cell.1 + step.1);
        } else if input[r][c] == "." || input[r][c] == "^" {
            if can_add_obstruction(
                &input,
                r,
                c,
                rotate_dir.get(&dir).unwrap(),
                &visited_obstructions,
                &dir_to_step,
                rows.try_into().unwrap(),
                cols.try_into().unwrap()
            ) {
                let next_step = *dir_to_step.get(dir).unwrap();
                let potential = (cell.0 + next_step.0, cell.1 + next_step.1);
                if potential != starting_cell {
                    new_obstructions.insert(potential);
                }
            }
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
    println!("Part Two answer: {}", new_obstructions.len());

}