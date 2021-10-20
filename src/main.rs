use std::fs::read_to_string;
use std::path::Path;
use std::cmp::{min,max};

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

fn puzzle_2015_2() -> (i64, i64){
    let input = get_input(2015, 2);
    let mut total_area = 0;
    let mut total_ribbon = 0;
    for line in input.split("\n") {
        let items: Vec<i64> = line.split("x").map(|e| e.parse::<i64>().expect("Invalid Input!")).collect();
        if items.len() == 3 {
            let xy = items[0] * items[1];
            let yz = items[1] * items[2];
            let xz = items[2] * items[0];
            let xp = (items[0])*2;
            let yp = (items[1])*2;
            let zp = (items[2])*2;
            let smallest_p = xp+yp+zp - max(max(xp,yp),zp);
            let bow = items[0] * items[1] * items[2];
            let smallest_a = min(min(xy,yz),xz);
            total_area += xy*2 + yz*2 + xz*2 + smallest_a;
            total_ribbon += bow + smallest_p
        }
    }
    (total_area, total_ribbon)
}

fn main() {
    println!("{:?}", puzzle_2015_2());
}
