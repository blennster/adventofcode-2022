use std::cmp;

pub fn aoc(lines: &Vec<String>) {
    let mut calories = vec![0];
    let mut i = 0;

    for line in lines {
        match line.as_str() {
            "" => {
                i += 1;
                calories.push(0);
            }
            &_ => {
                if let Ok(n) = line.parse::<i32>() {
                    calories[i] += n;
                }
            }
        }
    }

    // Sort the vector in descending order
    calories.sort();
    calories.reverse();

    p1(&calories);
    p2(&calories);
}

fn p1(calories: &[i32]) {
    // We can safely unwrap here because we know there is at least one element
    println!("Highest: {}", calories.first().unwrap());
}

fn p2(calories: &Vec<i32>) {
    // We want the top 3, but we don't want to panic if there are less than 3
    let len = cmp::min(3, calories.len());
    let top_n: i32 = calories[0..len].iter().sum();

    println!("Top {}: {}", len, top_n);
}
