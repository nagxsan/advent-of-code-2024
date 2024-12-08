use std::{collections::HashMap, fs, process::exit};

use itertools::{enumerate, Itertools};

fn read_input(path: &String) -> Result<String, String> {
    let result = fs::read_to_string(path);
    match result {
        Ok(content) => Ok(String::from(content)),
        Err(e) => Err(e.to_string())
    }
}

fn main() {
    let file_path = String::from("./src/bin/day-1-historian-hysteria/input.txt");
    let contents = read_input(&file_path);
    
    let mut input = match contents {
        Ok(v) => v,
        Err(e) => {
            println!("{e}");
            exit(1);
        }
    };

    // This next block of code is used to perform the following operations in order:
    // 1. Trim all the whitespaces from the input
    // 2. Split the input into a slice of strings based on the newline character
    // 3. For each element of the slice, split it on the basis of whitespaces (any number) and intersperse with a single whitespace i.e. replace multiple whitespaces with a single whitespace in between the numbers
    // 4. Append a newline to end of every element of slice
    // 5. Collect the slice into a single string
    input = input
    .trim()
    .lines()
    .flat_map(|line| {
        line.split_whitespace()
            .intersperse(" ")
            .chain(std::iter::once("\n"))
    })
    .collect();

    // This next line of code splits the input into a single slice of strings where the elements from the two lists are in alternating order
    let v: Vec<&str> = input.split_terminator(&[' ', '\n'][..]).collect();

    let mut list1: Vec<u32> = Vec::new();
    let mut list2: Vec<u32> = Vec::new();

    for (i, el) in enumerate(v) {
        let int_val = match el.parse::<u32>() {
            Ok(v) => v,
            Err(e) => {
                println!("Parsing error, incorrect input: {}", e.to_string());
                exit(1);
            }
        };

        if i % 2 == 0 {
            list1.push(int_val);
        } else {
            list2.push(int_val);
        }
    }

    list1.sort();
    list2.sort();

    let len = list1.len();

    let mut ans = 0;

    for i in 0..len {
        ans += list1[i].abs_diff(list2[i]);
    }

    // Part One solution:
    println!("Answer: {ans}");

    let mut hash_map: HashMap<u32, u32> = HashMap::new();

    for el in list2 {
        let freq = hash_map.entry(el).or_insert(0);
        *freq += 1;
    }

    let mut similarity_score = 0;

    for el in list1 {
        similarity_score += match hash_map.get_key_value(&el) {
            Some((el, val)) => (*val) * (*el),
            None => 0
        }
    }

    // Part Two solution:
    println!("Similarity Score: {similarity_score}");

}
