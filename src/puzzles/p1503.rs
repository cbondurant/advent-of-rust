use std::collections::HashMap;

use crate::get_data::get_data;

pub fn puzzle() -> (i64, i64) {
    let input = get_data(2015, 3);
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