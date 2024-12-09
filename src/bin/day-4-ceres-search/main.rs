use std::{fs, process::exit};

fn read_input(path: &str) -> Result<String, String> {
    match fs::read_to_string(path) {
        Ok(v) => Ok(v),
        Err(e) => Err(e.to_string())
    }
}

fn check_xmas(r: usize, c: usize, p: i32, q: i32, x: usize, y: usize, input: &Vec<Vec<&str>>) -> i32 {
    let xmas = String::from("XMAS");
    let mut row = r;
    let mut col = c;
    for i in 0..4 {
        if !(
            input[row][col] == xmas.chars().nth(i).unwrap().to_string()
        ) {
            return 0;
        }
        let new_row = row as i32 + p;
        let new_col = col as i32 + q;
        if i < 3 && (new_row < 0 || new_col < 0 || new_row >= x as i32 || new_col >= y as i32) {
            return 0;
        }
        row = new_row as usize;
        col = new_col as usize;
    }
    return 1;
}

fn main() {
    let file_path = "./src/bin/day-4-ceres-search/input.txt";
    let contents = match read_input(&file_path) {
        Ok(v) => v,
        Err(_e) => {
            println!("Error reading input data.");
            exit(1);
        }
    };

    let input: Vec<Vec<&str>> = contents
        .lines()
        .map(|line| line
            .split("")
            .filter(|ch| !ch.is_empty()) // remove empty strings
            .collect::<Vec<&str>>()
        )
        .collect();

    let mut ans = 0;

    let x = input.len();
    let y = input[0].len();
    
    let directions: Vec<(i32, i32)> = Vec::from([(-1, -1), (-1, 0), (-1, 1), (0, 1), (1, 1), (1, 0), (1, -1), (0, -1)]);

    for i in 0..x {
        for j in 0..y {
            if input[i][j] == "X" {
                for (p, q) in &directions {
                    ans += check_xmas(i, j, *p, *q, x, y, &input);
                }
            }
        }
    }

    println!("Answer: {ans}");

    let mut masx = 0;
    for i in 0..(x - 2) {
        for j in 0..(y - 2) {
            if input[i + 1][j + 1] == "A" {
                if input[i][j] == "M" && 
                    input[i][j + 2] == "M" &&
                    input[i + 2][j] == "S" &&
                    input[i + 2][j + 2] == "S" {
                    masx += 1;
                } else if input[i][j] == "M" && 
                    input[i + 2][j] == "M" &&
                    input[i][j + 2] == "S" &&
                    input[i + 2][j + 2] == "S" {
                    masx += 1
                } else if input[i][j + 2] == "M" && 
                    input[i + 2][j + 2] == "M" &&
                    input[i][j] == "S" &&
                    input[i + 2][j] == "S" {
                    masx += 1
                } else if input[i + 2][j + 2] == "M" && 
                    input[i + 2][j] == "M" &&
                    input[i][j] == "S" &&
                    input[i][j + 2] == "S" {
                    masx += 1
                }
            }
        }
    }

    println!("Answer: {masx}");

}