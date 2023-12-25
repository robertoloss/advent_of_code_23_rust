mod input;

#[derive(Copy, Clone,Debug)]
#[allow(dead_code)]
struct Pos {
    x: i32,
    y: i32,
}

fn is_plot(pos: &Pos, matrix: &Vec<Vec<String>> ) -> bool {
    return pos.x >= 0 &&
        pos.x < matrix[0].len() as i32 &&
        pos.y >= 0 &&
        pos.y < matrix.len() as i32 &&
        ( matrix[pos.y as usize][pos.x as usize] == String::from(".") || 
          matrix[pos.y as usize][pos.x as usize] == String::from("S") )
}

fn calc_plots_num(pos: &Pos, 
                   result: &mut i32, 
                   counter: i32,
                   max: i32,
                   matrix: &Vec<Vec<String>>,
                   pos_reached: &mut Vec<Pos>) {
    let pos_vec : Vec<Pos> = vec![
        Pos {x: pos.x-1, y: pos.y},
        Pos {x: pos.x, y: pos.y-1},
        Pos {x: pos.x+1, y: pos.y},
        Pos {x: pos.x, y: pos.y+1},
    ];
    for pos in pos_vec {
        if is_plot(&pos, &matrix) && counter < max {
            let num : i32 = counter + 1;
            //println!("");
            //println!("moving to pos: {:?}", pos);
            //println!("counter: {}", num);
            calc_plots_num(&pos, result, num, max, matrix, pos_reached);
        }
    }
    
    let len = pos_reached.len();
    let mut _new_position = true;
    for i in 0..len {
        if  pos_reached[i].x == pos.x && pos_reached[i].y == pos.y {
            //println!("been there, done that");
            _new_position = false;
            break
        }
    }
    if _new_position == true && counter == max {
        *result += 1;
        pos_reached.push(*pos);
        //println!("PLOT REACHED! {:?}", pos);
        println!("Result is now: {}", result);
    } 
    
}

fn create_matrix(input: String) -> Vec<Vec<String>> {
   let output : Vec<Vec<String>> = input
       .lines()
       .map(|line| line
            .chars()
            .map(|char| char
                 .to_string())
            .collect())
       .collect();
   return output
}

fn find_start(matrix: &Vec<Vec<String>>) -> Pos {
    let mut start : Pos = Pos {x:0, y:0};
    for y in 0..matrix.len() {
        for x in 0..matrix[y].len() {
            if matrix[y][x] == String::from("S") {
                start.x = x as i32;
                start.y = y as i32;
            }
        }
    }
    //println!("start : {:?}", start);
    return start
}
    
fn main() {
    let input = input::get_input();
    let matrix = create_matrix(input);
    let start = find_start(&matrix);
    let counter : i32 = 0;
    let mut result : i32 = 0;
    let max = input::get_max();
    let mut pos_reached : Vec<Pos> = vec![];
    calc_plots_num(&start, &mut result, counter, max, &matrix, &mut pos_reached);
    println!("Result : {}", result);
}
