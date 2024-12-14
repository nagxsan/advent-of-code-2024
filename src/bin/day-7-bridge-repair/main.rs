use std::{fs, process::exit};

fn calculate(target: i128, current_val: i128, idx: usize, num_of_operands: usize, operands: &Vec<i128>, is_answer_two: bool) -> bool {
    if idx == num_of_operands && target == current_val {
        return true;
    }

    if idx == num_of_operands {
        return false;
    }

    let add_result = calculate(target, current_val + operands[idx], idx + 1, num_of_operands, operands, is_answer_two);
    let mult_result = calculate(target, current_val * operands[idx], idx + 1, num_of_operands, operands, is_answer_two);

    let mut concat_result = false;

    if is_answer_two == true {
        let new_val = current_val.to_string() + &operands[idx].to_string();
        let new_val = new_val.parse::<i128>().unwrap();
        concat_result = calculate(target, new_val, idx + 1, num_of_operands, operands, is_answer_two);
    }

    return add_result || mult_result || concat_result;
}

fn main() {
    let input = match fs::read_to_string("./src/bin/day-7-bridge-repair/input.txt") {
        Ok(v) => v,
        Err(_e) => {
            println!("Error parsing input.");
            exit(1);
        }
    };

    let input = input.lines().map(|l| l).collect::<Vec<&str>>();

    let mut part_one_ans = 0;
    let mut part_two_ans = 0;

    for line in input {
        let equation = line.split(":").collect::<Vec<&str>>();
        let target = equation[0].parse::<i128>().unwrap();
        let operands = equation[1]
            .split_ascii_whitespace()
            .map(|op| op.parse::<i128>().unwrap())
            .collect::<Vec<i128>>();

        if calculate(target, 0, 0, operands.len(), &operands, false) {
            part_one_ans += target;
        }
        
        if calculate(target, 0, 0, operands.len(), &operands, true) {
            part_two_ans += target;
        }
    }

    println!("Part One answer: {part_one_ans}");
    println!("Part Two answer: {part_two_ans}");
}