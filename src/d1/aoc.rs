pub fn aoc(lines: &[String]) {
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

fn p2(calories: &[i32]) {
    let top_three: i32 = calories[0..3].iter().sum();

    println!("Top three: {}", top_three);
}
