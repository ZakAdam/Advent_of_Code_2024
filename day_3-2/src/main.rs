use std::fs::File;
use std::io::Read;
use regex::Regex;

fn main() -> std::io::Result<()> {
    println!("Hello, world!");

    let mut file = File::open("input.txt")?;
    let mut text = String::new();

    file.read_to_string(&mut text).expect("Cannot open file");

    let search_regex = Regex::new(r"mul\(\d+,\d+\)|do\(\)|don't\(\)").unwrap();

    let results:Vec<&str> = search_regex.find_iter(&text).map(|item| item.as_str()).collect();

    let mut sum_mul = 0;
    let mut count:bool = true;

    for line in results {
        if line == "do()" { count = true; continue;}
        if line == "don't()" { count = false; continue; }

        if count {
            let number: Vec<_> = line.split(",").collect();

            let number_1:u64 = number[0][4..].parse::<u64>().unwrap();
            let number_2:u64 = number[1][0..number[1].len() - 1].parse::<u64>().unwrap();
            //println!("{}", count);

            sum_mul += number_1 * number_2;
        }
    }

    println!("{}", sum_mul);

    Ok(())
}
