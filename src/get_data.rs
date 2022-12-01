use std::fs::read_to_string;
use std::path::Path;

pub fn get_data(year: i32, day: i32) -> String {
    read_to_string(Path::new(&format!("inputs/{}/{}.txt", year, day))).expect("")
}
