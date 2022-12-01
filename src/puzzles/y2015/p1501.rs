use crate::get_data::get_data;

pub fn puzzle() -> (i64, i64) {
    let input = get_data(2015, 1);
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