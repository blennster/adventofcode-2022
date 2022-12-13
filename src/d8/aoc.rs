struct Map {
    grid: Vec<Vec<u32>>,
}

#[derive(Debug)]
struct Visibility {
    top: bool,
    bottom: bool,
    left: bool,
    right: bool,
}

impl Visibility {
    fn new() -> Self {
        Visibility {
            top: true,
            bottom: true,
            left: true,
            right: true,
        }
    }

    fn is_visible(&self) -> bool {
        self.top || self.bottom || self.left || self.right
    }
}

impl Map {
    fn from_lines(lines: &[String]) -> Self {
        let mut matrix: Vec<Vec<u32>> = Vec::new();

        for line in lines {
            let mut row = Vec::new();
            for c in line.chars() {
                row.push(c.to_digit(10).unwrap());
            }

            matrix.push(row);
        }

        Map { grid: matrix }
    }

    fn visibility(&self, i: usize, j: usize) -> Visibility {
        let mut directions = Visibility::new();
        let value = self.grid[i][j];
        let m = self.grid.len();

        for n in 0..m {
            let row = self.grid[i][n];
            if n < j && row >= value {
                directions.left = false;
            }
            if n > j && row >= value {
                directions.right = false;
            }

            let col = self.grid[n][j];
            if n < i && col >= value {
                directions.top = false;
            }
            if n > i && col >= value {
                directions.bottom = false;
            }
        }

        directions
    }

    fn scenic_score(&self, i: usize, j: usize) -> i32 {
        let m = self.grid.len();
        let mut l_s = 0;
        let mut r_s = 0;
        let mut t_s = 0;
        let mut b_s = 0;
        let value = self.grid[i][j];

        for n in (0..j).rev() {
            let row = self.grid[i][n];
            l_s += 1;
            if row >= value {
                break;
            }
        }

        for n in 1 + j..m {
            let row = self.grid[i][n];
            r_s += 1;
            if row >= value {
                break;
            }
        }

        for n in (0..i).rev() {
            let col = self.grid[n][j];
            t_s += 1;
            if col >= value {
                break;
            }
        }

        for n in 1 + i..m {
            let col = self.grid[n][j];
            b_s += 1;
            if col >= value {
                break;
            }
        }

        l_s * r_s * t_s * b_s
    }

    fn scenic_score_matrix(&self) -> Vec<Vec<i32>> {
        let mut matrix = vec![];
        let len = self.grid.len();
        for i in 0..len {
            let mut m = vec![];
            for j in 0..len {
                let vis = self.scenic_score(i, j);
                m.push(vis);
            }
            matrix.push(m);
        }

        matrix
    }

    fn visibility_matrix(&self) -> Vec<Vec<Visibility>> {
        let mut matrix = vec![];
        let len = self.grid.len();
        for i in 0..len {
            let mut m = vec![];
            for j in 0..len {
                let vis = self.visibility(i, j);
                m.push(vis);
            }
            matrix.push(m);
        }

        matrix
    }
}

pub fn aoc(lines: &[String]) {
    let map = Map::from_lines(lines);
    p1(&map);
    p2(&map);
}

fn p1(map: &Map) {
    let vis_matrix = map.visibility_matrix();
    let vis_matrix: Vec<_> = vis_matrix
        .into_iter()
        .map(|x| {
            x.into_iter()
                .map(|y| match y.is_visible() {
                    true => Some(y),
                    false => None,
                })
                .collect::<Vec<Option<Visibility>>>()
        })
        .collect();

    let mut n_visible = 0;
    for (i, row) in vis_matrix.iter().enumerate() {
        for (j, col) in row.iter().enumerate() {
            print!("[{}, {}]: ", i, j);
            match col {
                Some(vis) => {
                    n_visible += 1;
                    println!("{:?}", vis)
                }
                None => println!("no vis"),
            }
        }
    }
    println!("Visible: {}", n_visible);
}

fn p2(map: &Map) {
    let scores = map.scenic_score_matrix();

    let highest = scores.into_iter().flatten().max().unwrap();

    println!("Highest score: {}", highest);
}
