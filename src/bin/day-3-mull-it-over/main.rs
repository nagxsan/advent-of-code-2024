use std::{fs, process::exit};

use regex::Regex;

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

    let re_mul = Regex::new(r"mul\((\d+,\d+)\)").unwrap();
    let re_do = Regex::new(r"do\(\)").unwrap();
    let re_dont = Regex::new(r"don\'t\(\)").unwrap();

    let mut matches_vec: Vec<(u32, String)> = vec![];
    
    for cap in re_mul.find_iter(&input) {
        matches_vec.push((cap.start().try_into().unwrap(), cap.as_str().to_string()));
    }

    for cap in re_do.find_iter(&input) {
        matches_vec.push((cap.start().try_into().unwrap(), cap.as_str().to_string()));
    }

    for cap in re_dont.find_iter(&input) {
        matches_vec.push((cap.start().try_into().unwrap(), cap.as_str().to_string()));
    }

    matches_vec.sort_by_key(|k| k.0);

    let mut do_flag= true;
    let mut ans = 0;
    for (_, m) in matches_vec {
        if m == "don't()" {
            do_flag = false;
        } else if m == "do()" {
            do_flag = true;
        } else {
            if do_flag {
                let m_slice = &m[4..m.len() - 1];
                let nums: Vec<i32> = m_slice
                    .split(',')
                    .map(|v| v.parse::<i32>().unwrap())
                    .collect();
                ans += nums[0] * nums[1];
            }
        }
    }

    println!("Answer: {ans}");
}