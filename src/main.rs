use std::fs::read_to_string;
use std::path::Path;

fn get_input(year: i32, day: i32) -> String{
    read_to_string(Path::new(&format!("inputs/{}-{}.txt", year, day))).expect("")
}

fn puzzle_2015_1() -> (i64, i64){
    let input = get_input(2015, 1);
    let mut floor = 0;
    let mut count = 0;
    let mut basecount = 0;
    for c in input.chars(){
        count += 1;
        match c {
            '(' => floor += 1,
            ')' => floor -= 1,
            '\n' => (),
            _ => panic!("Invalid Input!")
        }
        if (floor == -1)  & (basecount == 0) {
          basecount = count
        }
    }
    (floor, basecount)
}

fn main() {
    println!("{:?}", puzzle_2015_1());
}
