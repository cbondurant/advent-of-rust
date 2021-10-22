#![allow(dead_code)]
use std::cmp::{max, min};
use std::collections::HashMap;
use std::fs::read_to_string;
use std::path::Path;

fn get_input(year: i32, day: i32) -> String {
    read_to_string(Path::new(&format!("inputs/{}-{}.txt", year, day))).expect("")
}

fn puzzle_2015_1() -> (i64, i64) {
    let input = get_input(2015, 1);
    let mut floor = 0;
    let mut count = 0;
    let mut basecount = 0;
    for c in input.chars() {
        count += 1;
        match c {
            '(' => floor += 1,
            ')' => floor -= 1,
            '\n' => (),
            _ => panic!("Invalid Input!"),
        }
        if (floor == -1) & (basecount == 0) {
            basecount = count
        }
    }
    (floor, basecount)
}

fn puzzle_2015_2() -> (i64, i64) {
    let input = get_input(2015, 2);
    let mut total_area = 0;
    let mut total_ribbon = 0;
    for line in input.split("\n") {
        let items: Vec<i64> = line
            .split("x")
            .map(|e| e.parse::<i64>().expect("Invalid Input!"))
            .collect();
        if items.len() == 3 {
            let xy = items[0] * items[1];
            let yz = items[1] * items[2];
            let xz = items[2] * items[0];
            let xp = (items[0]) * 2;
            let yp = (items[1]) * 2;
            let zp = (items[2]) * 2;
            let smallest_p = xp + yp + zp - max(max(xp, yp), zp);
            let bow = items[0] * items[1] * items[2];
            let smallest_a = min(min(xy, yz), xz);
            total_area += xy * 2 + yz * 2 + xz * 2 + smallest_a;
            total_ribbon += bow + smallest_p
        }
    }
    (total_area, total_ribbon)
}

fn puzzle_2015_3() -> (i64, i64) {
    let input = get_input(2015, 3);
    let mut homes: HashMap<(i64, i64), bool> = HashMap::new();
    let mut robo_homes: HashMap<(i64, i64), bool> = HashMap::new();
    let (mut gx, mut gy) = (0, 0); // Ghost from part 1
    let (mut rx, mut ry) = (0, 0); // Robo from part 2
    let (mut sx, mut sy) = (0, 0); // Santa from part 2
    homes.insert((0, 0), true);
    robo_homes.insert((0, 0), true);
    let mut is_r = false;
    for c in input.chars() {
        match c {
            '>' => gx -= 1,
            '<' => gx += 1,
            '^' => gy += 1,
            'v' => gy -= 1,
            _ => (),
        }
        if is_r == true {
            match c {
                '>' => rx -= 1,
                '<' => rx += 1,
                '^' => ry += 1,
                'v' => ry -= 1,
                _ => (),
            }
        } else {
            match c {
                '>' => sx -= 1,
                '<' => sx += 1,
                '^' => sy += 1,
                'v' => sy -= 1,
                _ => (),
            }
        }
        is_r = !is_r;
        homes.entry((gx, gy)).or_insert(true);
        robo_homes.entry((rx, ry)).or_insert(true);
        robo_homes.entry((sx, sy)).or_insert(true);
    }
    (homes.len() as i64, robo_homes.len() as i64)
}

fn puzzle_2015_4() -> (i64, i64) {
    let input = "iwrupvqb";
    let mut i = 0;
    let fivezeroes = 0;
    loop {
        let attempt = format!("{}{}", input, i);
        // gotta force it to the right type
        //        let hash = md5::compute(attempt);
        //        if format!("{:x}", hash).starts_with("000000"){
        break;
        //        }
        i += 1;
    }
    (fivezeroes, i)
}

fn puzzle_2015_5() -> (i64, i64) {
    let input = get_input(2015, 5);
    let mut count = 0;
    let mut count2 = 0;
    for line in input.split('\n') {
        if line == "" {
            break;
        } // Deals with final newline causing problems
        let check1 = line
            .split(&['a', 'e', 'i', 'o', 'u'][..])
            .collect::<Vec<&str>>()
            .len()
            > 3;

        let mut check2 = false;
        let mut prev = ' ';
        for c in line.chars() {
            if c == prev {
                check2 = true;
                break;
            }
            prev = c
        }

        let check3 =
            line.contains("ab") | line.contains("cd") | line.contains("pq") | line.contains("xy");

        let mut check4 = false;
        for i in 0..line.len() - 2 {
            if line[i..i + 1] == line[i + 2..i + 3] {
                check4 = true;
                break;
            }
        }

        let mut check5 = false;
        for i in 0..line.len() - 3 {
            for j in i + 2..line.len() - 1 {
                if &line[i..i + 2] == &line[j..j + 2] {
                    check5 = true;
                    break;
                }
            }
        }

        if check1 & check2 & !check3 {
            count += 1;
        }

        if check4 & check5 {
            count2 += 1;
        }
    }
    (count, count2)
}

fn p15_6_parse_coords(expr: &str) -> Vec<Vec<usize>> {
    expr.split(" through ")
        .map(|coord| coord.split(',').map(|i| i.parse().unwrap()).collect())
        .collect::<Vec<Vec<usize>>>()
}

fn puzzle_2015_6() -> (i64, i64) {
    let input = get_input(2015, 6);
    let mut lights: Vec<bool> = vec![false; 1000 * 1000];
    let mut dimmerlights: Vec<i64> = vec![0; 1000 * 1000];
    for line in input.split("\n") {
        if line.starts_with("turn on") {
            let coords = p15_6_parse_coords(&line[8..]);
            for i in coords[0][0]..=coords[1][0] {
                for j in coords[0][1]..=coords[1][1] {
                    lights[i + j * 1000] = true;
                    dimmerlights[i + j * 1000] += 1;
                }
            }
        }
        if line.starts_with("turn off") {
            let coords = p15_6_parse_coords(&line[9..]);
            for i in coords[0][0]..=coords[1][0] {
                for j in coords[0][1]..=coords[1][1] {
                    lights[i + j * 1000] = false;
                    if dimmerlights[i + j * 1000] > 0 {
                        dimmerlights[i + j * 1000] -= 1;
                    }
                }
            }
        }
        if line.starts_with("toggle") {
            let coords = p15_6_parse_coords(&line[7..]);
            for i in coords[0][0]..=coords[1][0] {
                for j in coords[0][1]..=coords[1][1] {
                    lights[i + j * 1000] = !lights[i + j * 1000];
                    dimmerlights[i + j * 1000] += 2;
                }
            }
        }
    }
    let mut numon = 0;
    for i in lights.iter() {
        if *i {
            numon += 1
        }
    }
    (numon, dimmerlights.iter().sum())
}

fn main() {
    println!("{:?}", puzzle_2015_6());
}
