use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

fn main() {
    println!("Hello, world!");

    let file = File::open("input.txt").expect("Could not open");
    let reader = BufReader::new(file);

    let mut count = 0;
    let mut rows:Vec<Vec<char>> = vec![];

    for line in reader.lines() {
        let mut data:Vec<char> = vec![];

        for char in line.expect("line could not be read").chars() {
            data.push(char);
        }

        rows.push(data);
    }

    for (row_index, row) in rows.iter().enumerate() {
        for (column_index, column) in row.iter().enumerate() {
            if column == &'X' {
               if check_upper(&rows, row_index, column_index) { count += 1; }
               if check_lower(&rows, row_index, column_index, rows.len()) { count += 1; }
               if check_right(&rows, row_index, column_index, row.len()) { count += 1; }
               if check_left(&rows, row_index, column_index) { count += 1; }
               if check_left_upper(&rows, row_index, column_index) { count += 1; }
               if check_left_lower(&rows, row_index, column_index, rows.len()) { count += 1; }
               if check_right_upper(&rows, row_index, column_index, row.len()) { count += 1; }
               if check_right_lower(&rows, row_index, column_index, rows.len(), row.len()) { count += 1; }
            }
        }
    }

    print!("COUNT IS: {}", count);
}

fn check_upper(rows: &Vec<Vec<char>>, row:usize, column:usize) -> bool {
    if row < 3 { return false }

    let m_char = rows[row - 1][column];
    let a_char = rows[row - 2][column];
    let s_char = rows[row - 3][column];

    if m_char == 'M' && a_char == 'A' && s_char == 'S' { println!("UPPER: {} {} {}", m_char, a_char, s_char) ; return true }

    false
}

fn check_lower(rows: &Vec<Vec<char>>, row:usize, column:usize, rows_count:usize) -> bool {
    if row > (rows_count - 4) { return false }

    let m_char = rows[row + 1][column];
    let a_char = rows[row + 2][column];
    let s_char = rows[row + 3][column];

    if m_char == 'M' && a_char == 'A' && s_char == 'S' { println!("LOWER: {} {} {}", m_char, a_char, s_char) ; return true }

    false
}

fn check_right(rows: &Vec<Vec<char>>, row:usize, column:usize, columns_count:usize) -> bool {
    if column > (columns_count - 4) { return false }

    let m_char = rows[row][column + 1];
    let a_char = rows[row][column + 2];
    let s_char = rows[row][column + 3];

    if m_char == 'M' && a_char == 'A' && s_char == 'S' { println!("RIGTH: {} {} {}", m_char, a_char, s_char) ; return true }

    false
}

fn check_left(rows: &Vec<Vec<char>>, row:usize, column:usize) -> bool {
    if column < 3 { return false }

    let m_char = rows[row][column - 1];
    let a_char = rows[row][column - 2];
    let s_char = rows[row][column - 3];

    if m_char == 'M' && a_char == 'A' && s_char == 'S' { println!("LEFT: {} {} {}", m_char, a_char, s_char) ; return true }

    false
}

fn check_left_upper(rows: &Vec<Vec<char>>, row:usize, column:usize) -> bool {
    if column < 3 { return false }
    if row < 3 { return false }

    let m_char = rows[row - 1][column - 1];
    let a_char = rows[row - 2][column - 2];
    let s_char = rows[row - 3][column - 3];

    if m_char == 'M' && a_char == 'A' && s_char == 'S' { println!("LEFT UPPER: {} {} {}", m_char, a_char, s_char) ; return true }

    false
}

fn check_left_lower(rows: &Vec<Vec<char>>, row:usize, column:usize, rows_count:usize) -> bool {
    if column < 3 { return false }
    if row > rows_count - 4 { return false }

    let m_char = rows[row + 1][column - 1];
    let a_char = rows[row + 2][column - 2];
    let s_char = rows[row + 3][column - 3];

    if m_char == 'M' && a_char == 'A' && s_char == 'S' { println!("LEFT LOWER: {} {} {}", m_char, a_char, s_char) ; return true }

    false
}

fn check_right_upper(rows: &Vec<Vec<char>>, row:usize, column:usize, columns_count:usize) -> bool {
    if column > (columns_count - 4) { return false }
    if row < 3 { return false }

    let m_char = rows[row - 1][column + 1];
    let a_char = rows[row - 2][column + 2];
    let s_char = rows[row - 3][column + 3];

    if m_char == 'M' && a_char == 'A' && s_char == 'S' { println!("RIGHT UPPER: {} {} {}", m_char, a_char, s_char) ; return true }

    false
}

fn check_right_lower(rows: &Vec<Vec<char>>, row:usize, column:usize, rows_count:usize, columns_count:usize) -> bool {
    if column > (columns_count - 4) { return false }
    if row > (rows_count - 4) { return false }

    let m_char = rows[row + 1][column + 1];
    let a_char = rows[row + 2][column + 2];
    let s_char = rows[row + 3][column + 3];

    if m_char == 'M' && a_char == 'A' && s_char == 'S' { println!("RIGHT LOWER: {} {} {}", m_char, a_char, s_char) ; return true }

    false
}
