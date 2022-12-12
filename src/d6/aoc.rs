use std::collections::HashSet;

pub fn aoc(lines: &[String]) {
    let mut window = vec![];
    let mut start_points = vec![];

    // 4 for part one
    // 14 for part two
    let n = 14;

    for line in lines {
        for (i, c) in line.chars().enumerate() {
            if i > n {
                window.remove(0);
            }

            window.push(c);
            if window.iter().collect::<HashSet<_>>().len() > n {
                start_points.push(i);
                window.clear();
                break;
            }
        }
    }

    println!("{:?}", start_points);
}
