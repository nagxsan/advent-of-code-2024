use std::{collections::HashSet, fs, process::exit};

fn read_input(path: &str) -> Result<String, String> {
    match fs::read_to_string(path) {
        Ok(v) => Ok(v),
        Err(e) => Err(e.to_string())
    }
}

fn main() {
    let file_path = "./src/bin/day-5-print-queue/input.txt";
    let input = match read_input(&file_path) {
        Ok(v) => v,
        Err(_e) => {
            println!("Error reading input.");
            exit(1);
        }
    };

    let input: Vec<&str> = input.split("\n\n").collect();
    let rules: Vec<&str> = input[0].split('\n').collect();
    let updates: Vec<&str> = input[1].split('\n').collect();

    let rules = rules
        .into_iter()
        .map(|r| r.split('|').collect::<Vec<&str>>())
        .into_iter()
        .map(|r| (r[0].parse::<i32>().unwrap(), r[1].parse::<i32>().unwrap()))
        .collect::<Vec<(i32, i32)>>();

    let rule_pairs: HashSet<(i32, i32)> = rules.into_iter().collect();

    let mut part_one_ans = 0;
    let mut part_two_ans = 0;

    for update in updates {
        let page_nums: Vec<i32> = update
            .split(',')
            .map(|v| v.parse::<i32>().unwrap())
            .collect();
        let update_len = page_nums.len();
        let mut flag = true;
        for i in 0..update_len {
            for j in (i + 1)..update_len {
                if rule_pairs.contains(&(page_nums[j], page_nums[i])) {
                    flag = false;
                    break;
                }
            }
            if !flag {
                break;
            }
        }
        if flag {
            part_one_ans += page_nums[update_len / 2];
        } else {
            let mut corrected_page_nums: Vec<i32> = Vec::new();
            for i in 0..page_nums.len() {
                let num = page_nums[i];
                if corrected_page_nums.len() == 0 {
                    corrected_page_nums.push(num);
                } else {
                    let mut j = 0;
                    while j < corrected_page_nums.len() {
                        if rule_pairs.contains(&(num, corrected_page_nums[j])) {
                            break;
                        }
                        j += 1;
                    }
                    if j == corrected_page_nums.len() {
                        corrected_page_nums.push(num);
                    } else {
                        corrected_page_nums.insert(j, num);
                    }
                }
            }
            part_two_ans += corrected_page_nums[update_len / 2];
        }
    }

    println!("Part One answer: {part_one_ans}");
    println!("Part Two answer: {part_two_ans}");
}