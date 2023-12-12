mod input;

fn get_empty_cols_and_rows(input: &String) -> (Vec<i32>,Vec<i32>) {
    let mut cols : Vec<i32> = vec![];
    let mut rows : Vec<i32> = vec![];

    let mut not_cols : Vec<i32> = vec![];
    let mut not_rows : Vec<i32> = vec![];

    let mut y : i32 = 0;
    let mut x : i32 = 0;

    for c in input.chars() {
        if c == '#' { 
            if cols.contains(&y) {
                cols.retain(|&k| k != y);
            }
            if rows.contains(&x) {
                rows.retain(|&k| k != x);
            }
            not_cols.push(y);
            not_rows.push(x);
        } else if c == '.' {
            if !cols.contains(&y) && !not_cols.contains(&y) { cols.push(y) }
            if !rows.contains(&x) && !not_rows.contains(&x) { rows.push(x) }
        }
        x += 1;
        if c == '\n' {
            y += 1;
            x = 0;
        }
    }
    let result : (Vec<i32>,Vec<i32>) = (cols,rows);
    return result
}

fn expand_space(cols: Vec<i32>, rows: Vec<i32>, mut input: String) -> String {
    let mut x : i32 = 0;
    let mut y : i32 = 0;
    let mut row_length : i32 = 0;

    for c in input.chars() {
        if c == '\n' { break }
        row_length += 1;
    }
    let _initial_row_length = row_length;
    row_length += rows.len() as i32;
    ////println!("row_length : {}", row_length);
    let mut i = 0;
    while i < input.len() {
        if x == 0 && cols.contains(&y) { 
            let mut new_row = String::from("");
            for _i in 0..row_length {
                new_row.push('.');
            }
            new_row.push('\n');
            for _h in 0..rows.len() {
                new_row.push('.');
            }
            let row_clone = new_row.clone();
            input.insert_str(i,&row_clone); 
            i += ((row_length + 1)*2) as usize;
            y += 1;
            continue
        }
        if rows.contains(&x) {
            input.insert(i,'.');
            i += 1; 
        }
        x += 1;
        if input.chars().nth(i).unwrap() == '\n' {
            y += 1;
            x = 0;
        }
        i += 1;
    }
    return input
}

fn get_coords(input: String) -> Vec<(i32,i32)> {
    let mut coords : Vec<(i32,i32)> = vec![];
    let mut x : i32 = 0;
    let mut y : i32 = 0;
    for c in input.chars() {
        if c == '#' { coords.push((y,x)); }
        x += 1;
        if c == '\n' {
            y += 1;
            x = 0;
        }
    }
    return coords 
}

fn get_pairs(coords: Vec<(i32,i32)>) -> Vec<((i32,i32),(i32,i32))> {
    let mut pairs : Vec<((i32,i32),(i32,i32))> = vec![];
    for i in 0..coords.len() {
        for j in i+1..coords.len() {
            pairs.push((coords[i],coords[j]));
        }
    }
    return pairs
}

fn get_path(galaxy_1: (i32,i32), galaxy_2: (i32,i32)) -> i32 {
    return (galaxy_2.0 - galaxy_1.0) + (galaxy_2.1 - galaxy_1.1).abs() 
}

fn get_paths(pairs: Vec<((i32,i32),(i32,i32))>) -> Vec<i32> {
    let mut paths : Vec<i32> = vec![];
    for pair in pairs {
        paths.push(get_path(pair.0, pair.1));
    }
    return paths
}

fn sum_paths(paths: Vec<i32>) -> i32 {
    let mut sum : i32 = 0;
    for num in paths {
        sum += num;
    } 
    return sum
}

fn main() {
    let input = input::get_input();
    println!("input : \n{}", input);
    println!("");

    let (cols,rows) = get_empty_cols_and_rows(&input);
    println!("(cols,rows) : {:?}", (&cols,&rows));
    println!("");

    let new_input = expand_space(cols, rows, input);
    println!("new_input : \n{}", new_input);
    println!("");

    let coords = get_coords(new_input);
    println!("coords {:?}", coords);

    let pairs = get_pairs(coords);
    println!("pairs {:?}", pairs);
    println!("");

    let paths = get_paths(pairs);
    println!("paths {:?}", paths);
    println!("");

    let sum = sum_paths(paths);
    println!("Result: {}", sum);
}
