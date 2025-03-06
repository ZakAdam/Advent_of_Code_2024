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

    let mut state = "";
    let mut safe_reports_counts = 0;

    let mut selected_data:Vec<Vec<i32>> = Vec::new();

    for levels in data {
        let mut count_row = true;

        if levels[0] > levels[1] {
            state = "decrease";
        } else if levels[0] < levels[1] {
            state = "increase";
        } else {
            continue;
        }

        let mut previous_number:i32 = -1;

        for &number in &levels {
            if previous_number < 0 {
                previous_number = number;
                continue
            }
            
            if number == previous_number {
                println!("{:?}", levels);
                println!("{} - {} - {}", number, previous_number, state);
                count_row = false;
                break
            }
            if state == "increase" {
                if previous_number > number {
                    count_row = false;
                    break;
                }
            } else {
                if number > previous_number {
                    count_row = false;
                    break
                }
            }

            if (number - previous_number).abs() > 3 {
                count_row = false;
                break;
            }

            previous_number = number;
        }

        if count_row {
            safe_reports_counts += 1;
            selected_data.push(levels);
        }
    }

    println!("reports count: {}", safe_reports_counts);
    println!("selected_data: {:?}", selected_data);

    Ok(())
}
