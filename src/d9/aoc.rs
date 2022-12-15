use std::collections::HashSet;

struct RopeGrid {
    head: (i32, i32),
    knots: Vec<(i32, i32)>,
    tail_history: HashSet<(i32, i32)>,
}

impl RopeGrid {
    fn new(start: (i32, i32), n: i32) -> Self {
        let mut knots = Vec::new();
        for _ in 0..n {
            knots.push(start);
        }

        Self {
            head: start,
            knots,
            tail_history: HashSet::new(),
        }
    }

    fn move_head(&mut self, direction: &Direction, n: i32) {
        for _ in 0..n {
            match direction {
                Direction::Up => self.head.0 += 1,
                Direction::Down => self.head.0 -= 1,
                Direction::Right => self.head.1 += 1,
                Direction::Left => self.head.1 -= 1,
            }
            self.move_tail();
            // self.print_grid();
        }
    }

    fn dist(knot1: (i32, i32), knot2: (i32, i32)) -> i32 {
        let dx = i32::abs(knot1.0 - knot2.0);
        let dy = i32::abs(knot1.1 - knot2.1);
        i32::max(dx, dy)
    }

    fn delta(knot1: (i32, i32), knot2: (i32, i32)) -> (i32, i32) {
        let dx = i32::clamp(knot2.0 - knot1.0, -1, 1);
        let dy = i32::clamp(knot2.1 - knot1.1, -1, 1);

        (dx, dy)
    }

    fn move_tail(&mut self) {
        let len = self.knots.len();
        for i in 0..len {
            let other;
            let mut knot;

            if i == 0 {
                other = self.head;
                knot = &mut self.knots[0];
            } else {
                other = self.knots[i - 1];
                knot = &mut self.knots[i];
            }

            if Self::dist(*knot, other) > 1 {
                let (dx, dy) = Self::delta(*knot, other);
                knot.0 += dx;
                knot.1 += dy;
            }
        }

        self.tail_history.insert(*self.knots.last().unwrap());
    }

    #[allow(dead_code)]
    fn print_grid(&self) {
        const HEIGHT: i32 = 5;
        const WIDTH: i32 = 6;

        for i in (0..HEIGHT).rev() {
            for j in 0..WIDTH {
                let point = (i, j);
                let mut printout = ".".to_owned();
                for (i, knot) in self.knots.iter().enumerate() {
                    if *knot == point {
                        printout = (i + 1).to_string();
                        break;
                    }
                }

                if point == self.head {
                    printout = "H".to_owned();
                }

                print!("{}", printout);
            }
            println!();
        }

        println!();
    }
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl From<&str> for Direction {
    fn from(c: &str) -> Self {
        match c {
            "U" => Direction::Up,
            "D" => Direction::Down,
            "R" => Direction::Right,
            "L" => Direction::Left,
            _ => panic!(),
        }
    }
}

pub fn aoc(lines: &[String]) {
    p1(lines);
    p2(lines);
}

fn p1(lines: &[String]) {
    let mut grid = RopeGrid::new((0, 0), 1);

    for line in lines {
        let mut split = line.split(' ');

        let dir: Direction = split.next().unwrap().into();
        let n: i32 = split.next().unwrap().parse().unwrap();

        grid.move_head(&dir, n);
    }

    println!("Positions with one knot: {}", grid.tail_history.len());
}

fn p2(lines: &[String]) {
    const N: i32 = 9;
    let mut grid = RopeGrid::new((0, 0), N);

    for line in lines {
        let mut split = line.split(' ');

        let dir: Direction = split.next().unwrap().into();
        let n: i32 = split.next().unwrap().parse().unwrap();

        grid.move_head(&dir, n);
    }

    println!("Positions with {} knots: {}", N, grid.tail_history.len());
}
