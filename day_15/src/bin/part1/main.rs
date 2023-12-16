mod input;

fn hash_algo(current: u32, c: char) -> u32 {
    let mut new_value : u32 = current;
    let ascii = c as u32;
    
    new_value += ascii;
    new_value *= 17;
    new_value = new_value % 256;

    return new_value
}

fn get_hash_of_string(input: String) -> u32 {
    let mut output : u32 = 0;
    for c in input.chars() {
        output = hash_algo(output, c);
    }
    return output
}

fn get_vec_of_steps(input: String) -> Vec<String> {
    let res : Vec<String> = input
        .split(',')
        .map(|s| s.to_string())
        .collect();
    return res
}

fn main() {
    let input = input::get_input();
    //let result = get_hash_of_string(String::from("HASH"));
    let vec_of_steps = get_vec_of_steps(input);
    
    let mut result : u32 = 0;
    for s in vec_of_steps.iter() {
        result += get_hash_of_string(s.to_string());
    }

    println!("Result: {}", result);
}
