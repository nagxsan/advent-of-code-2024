use std::{fs, process::exit};

fn read_input(path: &String) -> Result<String, String> {
    let data = fs::read_to_string(path);
    match data {
        Ok(v) => Ok(String::from(v)),
        Err(e) => Err(e.to_string())
    }
}

fn is_sequence_valid(seq: &Vec<i32>) -> bool {
    let mut is_increasing: Option<bool> = None;
    for i in 1..seq.len() {
        let diff = seq[i] - seq[i - 1];

        match is_increasing {
            None => {
                if diff > 0 {
                    if diff < 1 || diff > 3 {
                        return false;
                    }
                    is_increasing = Some(true);
                } else if diff < 0 {
                    if diff < -3 || diff > -1 {
                        return false;
                    }
                    is_increasing = Some(false);
                } else {
                    return false;
                }
            },
            Some(true) => {
                if diff < 1 || diff > 3 {
                    return false;
                }
            },
            Some(false) => {
                if diff < -3 || diff > -1 {
                    return false;
                }
            }
        }
    }
    return true;
}

fn main() {
    let file_path = String::from("./src/bin/day-2-red-nosed-reports/input.txt");
    let contents = read_input(&file_path);

    let input = match contents {
        Ok(v) => v,
        Err(_e) => {
            println!("Error reading input.");
            exit(1);
        }
    };
    
    let sliced_input: Vec<&str> = input.split('\n').collect();

    let mut safe_reports_count = 0;

    for report in sliced_input {
        let mut flag = true;
        let mut is_increasing = 0;
        let mut prev_val = -1;
        let mut report_values_it = report.split_whitespace();
        while let Some(val) = report_values_it.next() {
            let curr_val = match val.parse::<i32>() {
                Ok(v) => v,
                Err(_e) => {
                    println!("Error parsing input str to integer, invalid input.");
                    exit(1);
                }
            };
            if prev_val == -1 {
                prev_val = curr_val;
            } else {
                if is_increasing == 0{
                    if curr_val > prev_val {
                        is_increasing = 1;
                    } else if curr_val < prev_val {
                        is_increasing = -1;
                    } else {
                        flag = false;
                        break;
                    }
                }

                if is_increasing == 1 && (curr_val - prev_val < 1 || curr_val - prev_val > 3) {
                    flag = false;
                    break;
                }
                if is_increasing == -1 && (prev_val - curr_val < 1 || prev_val - curr_val > 3) {
                    flag = false;
                    break;
                }
                prev_val = curr_val;
            }
        }

        if flag {
            safe_reports_count += 1;
        }
    }

    // Part One solution
    println!("Safe reports count: {}", safe_reports_count);

    let mut problem_dampened_safe_reports = 0;

    let sliced_input: Vec<&str> = input.split('\n').collect();
    for report in sliced_input {
        let report_values: Vec<i32> = report
            .split_whitespace()
            .map(|x| x.parse::<i32>().expect("Invalid parsing"))
            .collect();
        
        if is_sequence_valid(&report_values) {
            problem_dampened_safe_reports += 1;
        } else {
            for i in 0..report_values.len() {
                let mut new_report_values = report_values.to_vec();
                new_report_values.remove(i);
                if is_sequence_valid(&new_report_values) {
                    problem_dampened_safe_reports += 1;
                    break;
                }
            }
        }
    }

    // Part Two answer
    println!("Problem dampened safe reports count: {}", problem_dampened_safe_reports);

}