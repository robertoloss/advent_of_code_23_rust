mod input;
//use input::get_test_input_1;
//use input::get_input_1;

fn get_histories(input: String) -> Vec<Vec<i32>> {
    let result = input
        .lines()
        .map(|line| line
             .split_whitespace()
             .map(|s| s.parse::<i32>().unwrap())
             .collect())
        .collect();
    return result
}

fn not_all_zeroes(sequence: &Vec<i32>) -> bool {
    for num in sequence {
        if *num != 0 { return true }
    }
    return false 
}

fn value(history: Vec<i32>) -> i32 {
    let mut sequences : Vec<Vec<i32>> = vec![];
    sequences.push(history);
    while not_all_zeroes(&sequences.last().unwrap()) {
        let mut new_seq : Vec<i32> = vec![];
        for i in 1..sequences[sequences.len()-1].len() {
            new_seq.push(sequences.last().unwrap()[i] - sequences.last().unwrap()[i-1])
        }
        sequences.push(new_seq);
    }
    //println!("sequences.len() = {}", sequences.len());
    let mut p : usize = sequences.len() - 2;
    loop {
        let seq = sequences.clone();
        sequences[p].push( seq[p].last().unwrap() + seq[p+1].last().unwrap());
        if p == 0 { break }
        p -= 1;
    }
    return *sequences[0].last().unwrap() 
}

fn main() {
    let input = input::get_input_1();
    //println!("input: {}", input);
    let histories = get_histories(input);
    //println!("Histories : {:?}", histories);
    let mut total_sum : i32 = 0;
    for history in histories {
        if total_sum == 0 {println!("{}",total_sum)};
        total_sum += value(history);
    }
    println!("Result: {}", total_sum);
} 
