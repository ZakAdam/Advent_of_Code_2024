use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> std::io::Result<()> {
    println!("Hello, world!");

    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut data:Vec<Vec<i32>> = vec![];

    for line in reader.lines() {
        let row_data = line?.split(" ").map(|x| x.parse::<i32>().unwrap()).collect();
        data.push(row_data)
    }

    let mut rows_count = 0;

    for experiment in data {
        let mut count_row = false;

        for index in 0..experiment.len() {
            let (left, right) = experiment.split_at(index);
            let new_data = [&left, &right[1..]].concat();
            // println!("bez {}: {:?}", index, &new_data);

            if check_abs_difference(&new_data) && (check_increasing(&new_data) || check_decreasing(&new_data)) {
                //println!("{:?}", new_data);
                count_row = true;
                break;
            }
        }

        if count_row {
            rows_count += 1;
        } else {
            //println!("{:?}", experiment);
        }
    }

    println!("{}", rows_count);

    Ok(())
}

fn check_increasing(data: &Vec<i32>) -> bool {
    for index in 0..(data.len() - 1) {
        if data[index] > data[index + 1] { return false; }
    }

    true
}

fn check_decreasing(data: &Vec<i32>) -> bool {
    for index in 0..(data.len() - 1) {
        if data[index] < data[index + 1] { return false; }
    }

    true
}

fn check_abs_difference(data: &Vec<i32>) -> bool {
    for index in 0..data.len() - 1 {
        let difference = (data[index] - data[index + 1]).abs();

        if difference < 1 || difference > 3 { return false; }
    }

    println!("{:?}", data);
    println!("{} {}", data[0], data[1]);
    true
}
