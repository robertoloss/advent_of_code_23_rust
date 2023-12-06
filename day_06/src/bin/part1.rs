
fn how_many_ways(time: i32, record: i32) -> i32 {
    let mut ways_found = false;
    let mut num_of_ways = 0;
    for i in 1..time {
        if i as i32 * (time - i as i32) > record {
            num_of_ways += 1;
            if ways_found == false { ways_found = true };
        } else {
            if ways_found == true { break }
        }
    }
    return num_of_ways
}

fn get_time_record_pairs(input: String) -> Vec<(i32, i32)> {
    let mut result : Vec<(i32, i32)> = vec![];
    let vec_times_records : Vec<Vec<String>> = input
        .lines()
        .map(|line| {line
            .to_string()
            .split(':')
            .nth(1)
            .unwrap()
            .to_string()
            .split_whitespace()
            .map(|s| s.to_string())
            .collect()
        })
        .collect();
    for i in 0..vec_times_records[0].len() {
        let mut pair : (i32,i32) = (0,0);
        pair.0 = vec_times_records[0][i].parse::<i32>().unwrap();
        pair.1 = vec_times_records[1][i].parse::<i32>().unwrap();
        result.push(pair);
    }

    return result
}

fn main() {
    let input = String::from("Time:        41     96     88     94
Distance:   214   1789   1127   1055");

    let time_record_pairs = get_time_record_pairs(input);

    let mut vec_ways : Vec<i32> = vec![];


    for pair in time_record_pairs.iter() {
        vec_ways.push(how_many_ways(pair.0, pair.1))
    }


    let result = vec_ways
        .iter()
        .fold(1, |acc, &x| acc*x);
    
    println!("result : {}", result);


}
