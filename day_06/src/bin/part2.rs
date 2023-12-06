
fn how_many_ways(time: i64, record: i64) -> i64 {
    let mut ways_found = false;
    let mut num_of_ways = 0;
    for i in 1..time {
        if i as i64 * (time - i as i64) > record {
            num_of_ways += 1;
            if ways_found == false { ways_found = true };
        } else {
            if ways_found == true { break }
        }
    }
    return num_of_ways
}

fn get_time_record(input: String) -> (i64,i64) {
    let mut result : (i64,i64) = (0,0);
    let vec_time_record : Vec<String> = input
        .lines()
        .map(|line| {line
            .to_string()
            .split(':')
            .nth(1)
            .unwrap()
            .to_string()
            .split_whitespace()
            .fold(String::from(""), |acc, s| acc + s)})
        .collect();
    result.0 = vec_time_record[0].parse::<i64>().unwrap();
    result.1 = vec_time_record[1].parse::<i64>().unwrap();

    return result
}

fn main() {
    let input = String::from("Time:        41     96     88     94
Distance:   214   1789   1127   1055");

    let time_record_pair = get_time_record(input);

    let result = how_many_ways(time_record_pair.0, time_record_pair.1);
   
    println!("result : {}", result);
}
