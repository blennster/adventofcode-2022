struct Rucksack {
    sack: String,
    compartments: (String, String),
}

impl Rucksack {
    fn from_string(s: &String) -> Rucksack {
        let len = s.len();
        let p1 = &s[..len / 2];
        let p2 = &s[len / 2..];
        Rucksack {
            sack: s.clone(),
            compartments: (p1.to_owned(), p2.to_owned()),
        }
    }

    fn find_common(&self) -> char {
        for a in self.compartments.0.chars() {
            if self.compartments.1.contains(a) {
                return a;
            }
        }

        panic!();
    }

    fn find_common_with_others(&self, others: &[Rucksack]) -> char {
        for a in self.sack.chars() {
            let has_common = others.iter().filter(|&other| other.sack.contains(a));

            if has_common.count() == others.len() {
                return a;
            }
        }

        panic!();
    }
}

fn priority(c: &char) -> u32 {
    if c.is_ascii_uppercase() {
        return *c as u32 - 'A' as u32 + 27;
    } else if c.is_ascii_lowercase() {
        return *c as u32 - 'a' as u32 + 1;
    }

    panic!();
}

pub fn aoc(lines: &[String]) {
    p1(lines);
    p2(lines);
}

fn p1(lines: &[String]) {
    let sum = lines.iter().fold(0, |acc, line| {
        let rucksack = Rucksack::from_string(line);
        let common = rucksack.find_common();
        acc + priority(&common)
    });

    println!("Sum: {}", sum);
}

fn p2(lines: &[String]) {
    let mut sum = 0;
    for i in 0..lines.len() / 3 {
        let mut sacks = vec![];

        for j in 0..3 {
            let rucksack = Rucksack::from_string(&lines[i * 3 + j]);
            sacks.push(rucksack);
        }

        let common = sacks[0].find_common_with_others(&sacks[1..]);
        sum += priority(&common);
    }

    println!("Sum: {}", sum);
}
