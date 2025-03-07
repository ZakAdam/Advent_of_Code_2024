use std::fs::File;
use std::io::Read;
use regex::Regex;

fn main() -> std::io::Result<()> {
    println!("Hello, world!");

    let mut file = File::open("input.txt")?;
    let mut text = String::new();
    file.read_to_string(&mut text).expect("TODO: panic message");

    let search_regex = Regex::new(r"mul\(\d+,\d+\)").unwrap();
    //let search_regex = Regex::new(r"mul\(").unwrap();
    //let mut results = vec![];

    let results:Vec<_> = search_regex.find_iter(&text).map(|item| item.as_str()).collect();

    let mut sum_mul:u64 = 0;

    for line in results {
        let number: Vec<_> = line.split(",").collect();

        let number_1:u64 = number[0][4..].parse::<u64>().unwrap();
        let number_2:u64 = number[1][0..number[1].len() - 1].parse::<u64>().unwrap();
        println!("{} {}", number_1, number_2);
        
        sum_mul += number_1 * number_2;
    }
    
    println!("Sum: {}", sum_mul);

    Ok(())
}
