mod input;
use std::cmp;

#[allow(dead_code)]
struct Point {
    x: i32,
    y: i32
}

fn generate_matrix(input: String) -> Vec<Vec<String>> {
    return input.
        lines()
        .map(|line| line
             .chars()
             .map(|char| char
                  .to_string())
             .collect())
        .collect()
}

fn next_step(matrix: Vec<Vec<String>>, num: u8) -> Vec<Vec<String>> {
    let mut new_matrix : Vec<Vec<String>> = matrix.clone();
    for i in 0..matrix.len() {
        for j in 0..matrix[i].len() {
            if matrix[i][j] == (num - 1).to_string() || 
               matrix[i][j] == String::from("S") {
                new_matrix[i][j] = String::from(".");
                let mut _ii : usize = 0;
                if i == 0 {
                    _ii = 1;
                } else {
                    _ii = i;
                }
                let mut _jj : usize = 0;
                if j == 0 {
                    _jj = 1;
                } else {
                    _jj = j;
                }
                for y in cmp::max(0, _ii-1)..cmp::min(i+2,matrix.len()) {
                    for x in cmp::max(0,_jj-1)..cmp::min(j+2,matrix[j].len()) {
                        if ( matrix[y][x] == String::from(".") ||
                             matrix[y][x] == (num -1 ).to_string()) && 
                           ( ((y == i) || (x == j)) && !((y == i) && (x == j)) ) {
                            new_matrix[y][x] = num.to_string();
                        } else if matrix[y][x] == String::from('S') {
                            new_matrix[y][x] = String::from('.');
                        } 
                    }
                }
            } 
        }
    }
    return new_matrix
}


fn main() {
    let input = input::get_input();
    let max = input::get_max();
    let matrix = generate_matrix(input);

    for vec in &matrix {
        println!("{:?}", vec);
    }
    let mut counter : u8 = 1;
    let mut new_matrix = matrix.clone();
    
    while counter <= max as u8 {
        new_matrix = next_step(new_matrix, counter); 
        counter += 1;
        println!("");
        for vec in &new_matrix {
            println!("{:?}", vec);
        }
    }
    let mut result = 0;
    for y in 0..new_matrix.len() {
        for x in 0..new_matrix[y].len() {
            if new_matrix[y][x] == (counter - 1).to_string() {
                result += 1;
            }
        }
    }
    println!("Result : {}", result);
}
    
