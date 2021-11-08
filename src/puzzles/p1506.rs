use crate::get_data::get_data;

fn parse_coords(expr: &str) -> Vec<Vec<usize>> {
    expr.split(" through ")
        .map(|coord| coord.split(',').map(|i| i.parse().unwrap()).collect())
        .collect::<Vec<Vec<usize>>>()
}

pub fn puzzle() -> (i64, i64) {
    let input = get_data(2015, 6);
    let mut lights: Vec<bool> = vec![false; 1000 * 1000];
    let mut dimmerlights: Vec<i64> = vec![0; 1000 * 1000];
    for line in input.split("\n") {
        if line.starts_with("turn on") {
            let coords = parse_coords(&line[8..]);
            for i in coords[0][0]..=coords[1][0] {
                for j in coords[0][1]..=coords[1][1] {
                    lights[i + j * 1000] = true;
                    dimmerlights[i + j * 1000] += 1;
                }
            }
        }
        if line.starts_with("turn off") {
            let coords = parse_coords(&line[9..]);
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
            let coords = parse_coords(&line[7..]);
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