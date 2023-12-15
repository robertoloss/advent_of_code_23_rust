mod input;

fn get_vecs_and_length(input: String) -> Vec<Vec<char>> {
    let mut result : Vec<Vec<char>> = vec![];
    let mut i = 0;
    let mut p = 0;
    let vec : Vec<char> = vec![];
    result.push(vec);
    while p < input.len() {
        if input.chars().nth(p).unwrap() == '\n' {
            i = 0;
            p += 1;
            continue
        }
        if result.len() < i+1 {
            let vec : Vec<char> = vec![];
            result.push(vec);
        }
        result[i].push(input.chars().nth(p).unwrap());
        i += 1;
        p += 1;
    }
    return result
}

fn get_single_load(mut vec: Vec<char>, length: usize) -> i32 {
    let mut left : usize = 0;
    let mut _right : usize = 0;
    let mut load : i32 = 0;
    'outer: while left < length  {
       if vec[left] == '#' {
            left +=1;
            continue
       } 
       if vec[left] == 'O' {
           load += (length - left) as i32;
           left += 1;
           continue
       }
       _right = left + 1;
       while _right < length {
           if vec[_right] == 'O' {
               load += (length - left) as i32;
               vec.swap(left, _right);
               _right += 1;
               left += 1;
               continue
           }
           if vec[_right] == '#' {
               left = _right+1;
               continue 'outer;
           }
           _right += 1;
       }
       break
    }
    return load
}



fn main() {
    let input = input::_get_input();
    let tilted_vecs = get_vecs_and_length(input);

    let mut _sum : i32 = 0;
    for vec in tilted_vecs.iter() {
        _sum += get_single_load(vec.clone(), vec.len());
    }

    println!("Result {}", _sum);
}
