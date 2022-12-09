struct Range {
    min: u32,
    max: u32,
}

impl Range {
    fn from_str(s: &str) -> Self {
        let mut parts = s.split('-');
        let min = parts.next().unwrap().parse().unwrap();
        let max = parts.next().unwrap().parse().unwrap();
        Self { min, max }
    }

    fn includes(&self, other: &Range) -> bool {
        self.min <= other.min && other.max <= self.max
    }

    fn in_range(&self, n: u32) -> bool {
        self.min <= n && n <= self.max
    }

    fn overlaps(&self, other: &Range) -> bool {
        // One might also check that max is in range
        self.in_range(other.min) || other.in_range(self.min)
    }
}

pub fn aoc(lines: &[String]) {
    let contained: Vec<(Range, Range)> = lines
        .iter()
        .map(|l| {
            let mut parts = l.split(',');
            let r1 = Range::from_str(parts.next().unwrap());
            let r2 = Range::from_str(parts.next().unwrap());
            (r1, r2)
        })
        .collect();

    p1(&contained);
    p2(&contained);
}

fn p1(lines: &[(Range, Range)]) {
    let contained = lines.iter().fold(0, |acc, (r1, r2)| {
        if r1.includes(r2) || r2.includes(r1) {
            acc + 1
        } else {
            acc
        }
    });

    println!("Number contained: {}", contained);
}

fn p2(lines: &[(Range, Range)]) {
    let overlaps = lines.iter().fold(
        0,
        |acc, (r1, r2)| {
            if r1.overlaps(r2) {
                acc + 1
            } else {
                acc
            }
        },
    );

    println!("Number overlaps: {}", overlaps);
}
