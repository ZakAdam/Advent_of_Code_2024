use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> std::io::Result<()> {
    println!("Hello, world!");

    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    let mut left_side:Vec<i32> = vec![];

    let mut right_values_map:HashMap<i32, i32> = HashMap::new();

    for line in reader.lines() {
        let numbers = line?;
        let new_numbers = numbers.split("   ").collect::<Vec<&str>>();
        left_side.push(new_numbers[0].parse::<i32>().unwrap());
        *right_values_map.entry(new_numbers[1].parse::<i32>().unwrap()).or_insert(0) += 1;
    }

    let mut similarity_score:i32 = 0;

    for number in left_side {
        let similarity = right_values_map.get(&number);
        if similarity.is_none() {
            continue;
        } else {
            println!("{} -> {}", number, similarity.unwrap());
            similarity_score += number * similarity.unwrap();
        }
    }

    println!("Sum of differences is: {}", similarity_score);

    Ok(())
}
