use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> std::io::Result<()> {
    println!("Hello, world!");

    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    let mut left_side:Vec<i32> = vec![];
    let mut right_side:Vec<i32> = vec![];

    for line in reader.lines() {
        let numbers = line?;
        //println!("{:?}", numbers);
        let new_numbers = numbers.split("   ").collect::<Vec<&str>>();
        left_side.push(new_numbers[0].parse::<i32>().unwrap());
        right_side.push(new_numbers[1].parse::<i32>().unwrap());
    }
    // let mut data = vec![];
    // file.read_to_end(&mut data)?;
    println!("{}", left_side[0]);
    println!("{}", left_side[1]);
    println!("{}", left_side[2]);

    left_side.sort();
    right_side.sort();

    println!("{}", left_side[0]);
    println!("{}", left_side[1]);
    println!("{}", left_side[2]);

    let mut sum:i32 = 0;
    let mut i = 0;

    for number in left_side {
        let difference:i32 = (number - right_side[i]).abs();
        sum += difference;
        i += 1;
    }

    println!("{}", i);
    println!("Sum of differences is: {}", sum);

    Ok(())
}
