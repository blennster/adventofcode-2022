use std::str::FromStr;

struct Screen {
    width: usize,
    lines: usize,
    pos: usize,
    buffer: Vec<bool>,
}

impl Screen {
    fn new(width: usize, lines: usize) -> Self {
        let buf = (0..width * lines).map(|_| false).collect();
        Screen {
            width,
            lines,
            pos: 0,
            buffer: buf,
        }
    }

    fn draw(&mut self, lit: bool) {
        self.buffer[self.pos] = lit;
        self.pos += 1 % (self.width * self.lines);
    }

    fn print(&self) {
        for (i, b) in self.buffer.iter().enumerate() {
            let output = match b {
                true => "#",
                false => ".",
            };

            print!("{output}");
            if (i + 1) % self.width == 0 {
                println!();
            }
        }
    }
}

struct Cpu<'a> {
    cycle: i32,
    // Single register X
    x: i32,
    signal_strength: Vec<i32>,
    screen: Option<&'a mut Screen>,
}

impl<'a> Cpu<'a> {
    fn new(screen: Option<&'a mut Screen>) -> Self {
        Cpu {
            cycle: 0,
            x: 1,
            signal_strength: Vec::new(),
            screen,
        }
    }

    fn cycle(&mut self) {
        if let Some(screen) = &mut self.screen {
            let is_lit = ((self.cycle % screen.width as i32) - self.x).abs() < 2;
            screen.draw(is_lit);
        }
        self.cycle += 1;
        self.signal_strength.push(self.cycle * self.x);
    }

    fn exec(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::Noop => self.cycle(),
            Instruction::Add(x) => {
                self.cycle();
                self.cycle();
                self.x += x;
            }
        }
    }
}

enum Instruction {
    Noop,
    Add(i32),
}

impl FromStr for Instruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(' ');
        let instruction = split.next();

        if instruction == None {
            return Err("no instruction provided".to_owned());
        }

        let instruction = instruction.unwrap();

        match instruction {
            "noop" => Ok(Instruction::Noop),
            "addx" => match split.next() {
                Some(x) => match x.parse() {
                    Ok(x) => Ok(Instruction::Add(x)),
                    Err(_) => Err("missing x".to_owned()),
                },
                None => Err("".to_owned()),
            },
            _ => Err("unknown instruction".to_owned()),
        }
    }
}

pub fn aoc(lines: &[String]) {
    const WIDTH: usize = 40;
    const LINES: usize = 6;
    let mut screen = Screen::new(WIDTH, LINES);
    let mut cpu = Cpu::new(Some(&mut screen));

    for line in lines {
        let instruction: Instruction = line.parse().unwrap();

        cpu.exec(instruction);
    }

    p1(&cpu);
    p2(&screen);
}

fn p1(cpu: &Cpu) {
    let p = [20, 60, 100, 140, 180, 220];

    for i in p {
        println!("Strength @{i}: {}", cpu.signal_strength[i - 1]);
    }

    let p = p.map(|i| cpu.signal_strength[i - 1]);
    let tot: i32 = p.iter().sum();
    println!("Total: {tot}");
}

fn p2(screen: &Screen) {
    screen.print();
}
