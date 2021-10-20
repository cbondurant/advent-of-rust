use std::fs::read_to_string;
use std::path::Path;

fn get_input(year: i32, day: i32) -> String{
    read_to_string(Path::new(&format!("inputs/{}-{}.txt", year, day))).expect("")
}

fn puzzle_2018_01(input : &str) -> i64{
    let mut floor = 0;
    let mut count = 0;
    for c in input.chars(){
        count += 1;
        match c {
            '(' => floor += 1,
            ')' => floor -= 1,
            '\n' => (),
            _ => panic!("Invalid Input!")
        }
        if floor == -1 {
          break
        }
    }
    count
}

fn main() {
    println!("{}", puzzle_2018_01(&get_input(2018,01)));
}
