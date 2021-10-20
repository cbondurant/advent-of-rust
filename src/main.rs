#![allow(dead_code)]
use std::fs::read_to_string;
use std::path::Path;
use std::cmp::{min,max};
use std::collections::HashMap;

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

fn puzzle_2015_3() -> (i64, i64){
    let input = get_input(2015, 3);
    let mut homes: HashMap<(i64, i64), bool> = HashMap::new();
    let mut robo_homes: HashMap<(i64, i64), bool> = HashMap::new();
    let (mut gx,mut gy) = (0,0); // Ghost from part 1
    let (mut rx,mut ry) = (0,0); // Robo from part 2
    let (mut sx,mut sy) = (0,0); // Santa from part 2
    homes.insert((0,0), true);
    robo_homes.insert((0,0), true);
    let mut isR = false;
    for c in input.chars(){
        match c {
            '>' => gx-=1,
            '<' => gx+=1,
            '^' => gy+=1,
            'v' => gy-=1,
            _ => ()
        }
        if isR == true {
            match c {
                '>' => rx-=1,
                '<' => rx+=1,
                '^' => ry+=1,
                'v' => ry-=1,
                _ => ()
            }
        }
        else{
            match c {
                '>' => sx-=1,
                '<' => sx+=1,
                '^' => sy+=1,
                'v' => sy-=1,
                _ => ()
            }
        }
        isR = !isR;
        homes.entry((gx,gy)).or_insert(true);
        robo_homes.entry((rx,ry)).or_insert(true);
        robo_homes.entry((sx,sy)).or_insert(true);
    }
    (homes.len() as i64, robo_homes.len() as i64)
}

fn main() {
    println!("{:?}", puzzle_2015_3());
}
