fn parse_config(lines: &[String]) -> Vec<Vec<char>> {
    let n = lines[0]
        .trim()
        .split(' ')
        .last()
        .unwrap()
        .parse::<u32>()
        .unwrap();

    let mut stacks = vec![];

    for _ in 0..n {
        stacks.push(vec![]);
    }

    for line in &lines[1..] {
        let chars: Vec<_> = line.chars().collect();
        for (i, stack) in stacks.iter_mut().enumerate() {
            let c = chars[(1 + 4 * i) as usize];

            if !c.is_whitespace() {
                stack.push(c);
            }
        }
    }

    stacks
}

pub fn aoc(lines: &[String]) {
    let mut config_lines = vec![];
    let mut j = 0;

    for (i, line) in lines.iter().enumerate() {
        if line.is_empty() {
            j = i;
            break;
        }
        config_lines.push(line.clone());
    }

    // It is easier to parse the config in reverse
    config_lines.reverse();

    let move_lines: Vec<String> = lines.iter().skip(j + 1).map(String::to_owned).collect();

    p1(parse_config(&config_lines), &move_lines);
    p2(parse_config(&config_lines), &move_lines);
}

fn p1(mut stacks: Vec<Vec<char>>, lines: &[String]) {
    for line in lines {
        let split: Vec<&str> = line.split(' ').collect();
        let move_n = split[1].parse().unwrap();
        let move_from = split[3].parse::<u32>().unwrap() - 1;
        let move_to = split[5].parse::<u32>().unwrap() - 1;

        for _ in 0..move_n {
            let char = stacks[move_from as usize].pop().unwrap();
            stacks[move_to as usize].push(char);
        }
    }

    for stack in stacks {
        print!("{}", stack.last().unwrap());
    }
    println!();
}

fn p2(mut stacks: Vec<Vec<char>>, lines: &[String]) {
    for line in lines {
        let split: Vec<&str> = line.split(' ').collect();
        let move_n = split[1].parse().unwrap();
        let move_from = split[3].parse::<u32>().unwrap() - 1;
        let move_to = split[5].parse::<u32>().unwrap() - 1;

        let mut intermediate = vec![];
        for _ in 0..move_n {
            let char = stacks[move_from as usize].pop().unwrap();
            intermediate.push(char);
        }
        intermediate.reverse();
        stacks[move_to as usize].append(&mut intermediate);
    }

    for stack in stacks {
        print!("{}", stack.last().unwrap());
    }
    println!();
}
