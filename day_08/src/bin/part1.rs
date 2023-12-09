mod input1;
use std::collections::HashMap;


fn get_instructions_and_map(input: &String) -> 
    (String, HashMap<String, Vec<String>>)
{
    let instructions = input
        .lines()
        .nth(0)
        .unwrap()
        .to_string();

    //println!("{}", instructions);
    //println!("{}", input);
    
    let mut map: HashMap<String, Vec<String>> = HashMap::new();

    let map_values : Vec<Vec<String>> = input
        .split_once('\n')
        .unwrap()
        .1
        .trim()
        .lines()
        .map(|line| line
             .split('=')
             .nth(1)
             .unwrap()
             .trim()
             .trim_matches(|c| c == '(' || c == ')')
             .split(',')
             .map(|s| s.to_string().trim().to_string())
             .collect())
        .collect();

    let map_keys : Vec<String> = input
        .split_once('\n')
        .unwrap()
        .1
        .trim()
        .lines()
        .map(|line| line
             .split('=')
             .nth(0)
             .unwrap()
             .trim()
             .to_string())
        .collect();

    //println!("map_keys : {:?}", map_keys);
    //println!("map_values : {:?}", map_values);

    for i in 0..map_keys.len() {
        map.insert(map_keys[i].clone(), map_values[i].clone());
    }
                

    return (instructions, map)
}

fn get_vec_of_instructions(instructions: String) -> Vec<usize> {
    let mut output : Vec<usize> = vec![];
    for s in instructions.chars() {
        if s == 'R' { output.push(1); }
        else { output.push(0); }
    }
    return output
}

fn main() {
    let input = input::get_input();

    let (instructions, map) = get_instructions_and_map(&input);

    let mut counter : i32 = 0;

    let mut current_node = String::from("AAA");
    let voi = get_vec_of_instructions(instructions);
    let mut voi_pointer : usize = 0;

    while current_node != String::from("ZZZ") {
        //println!("currentonode {}", current_node);
        current_node = map.get(&current_node).unwrap()[voi[voi_pointer]].to_string();
        counter += 1;
        voi_pointer += 1;
        if voi_pointer == voi.len() {
            voi_pointer = 0;
        }
    }

    println!("result : {}", counter);


}
