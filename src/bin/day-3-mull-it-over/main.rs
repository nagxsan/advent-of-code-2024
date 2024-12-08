use std::{fs, process::exit};

use regex::{Regex, RegexSet};

fn read_input(path: &String) -> Result<String, String> {
    let data = match fs::read_to_string(path) {
        Ok(v) => Ok(v),
        Err(e) => Err(e.to_string())
    };
    data
}

fn main() {
    let file_path = String::from("./src/bin/day-3-mull-it-over/input.txt");
    let contents = read_input(&file_path);

    let input = match contents {
        Ok(v) => v,
        Err(_e) => {
            println!("Error reading input file.");
            exit(1);
        }
    };

    let re = Regex::new(r"mul\((\d+,\d+)\)").unwrap();

    let mut ans= 0;

    for cap in re.captures_iter(&input) {
        let nums: Vec<i32> = cap[1]
            .split(',')
            .map(|x| x.parse::<i32>().expect("Invalid parse"))
            .collect();
        
        ans += nums[0] * nums[1];
    }

    println!("Answer: {ans}");
}