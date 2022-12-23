use crate::{regex, PuzzleSolver};
use std::fmt::Formatter;

pub struct Day10 {
    instructions: Vec<Instruction>,
}

impl PuzzleSolver for Day10 {
    fn with_input(input: String) -> Self {
        let instructions = input.lines().map(Instruction::from_line).collect();
        Self { instructions }
    }

    fn part1(&self) -> String {
        let cycles = Cycles::new(&self.instructions);

        cycles
            .skip(19) // start at the 20th cycle
            .step_by(40) // use every 40th after that
            .map(|signal| signal.strength)
            .sum::<i32>()
            .to_string()
    }

    fn part2(&self) -> String {
        let rendered = Cycles::new(&self.instructions).render();

        print!("{rendered}");

        rendered
    }
}

#[derive(Debug, Clone, Copy)]
enum Instruction {
    Noop,
    Add(i32),
}

impl Instruction {
    fn from_line(line: &str) -> Self {
        let noop_regex = regex!(r#"^noop$"#);
        let add_regex = regex!(r#"^addx (?P<count>-?\d+)"#);
        if noop_regex.is_match(line) {
            Self::Noop
        } else if let Some(caps) = add_regex.captures(line) {
            let count = caps.name("count").unwrap().as_str().parse().unwrap();
            Self::Add(count)
        } else {
            panic!("invalid line: {line}");
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Signal {
    cycle: usize,
    strength: i32,
    rendering: bool,
}

#[derive(Clone, Copy)]
struct Cycles<'i> {
    register: i32,
    cycle: usize,
    instruction_ptr: usize,
    pending_add: Option<i32>,
    instructions: &'i [Instruction],
}

impl<'i> Cycles<'i> {
    fn new(instructions: &'i [Instruction]) -> Self {
        Self {
            register: 1,
            cycle: 1,
            instruction_ptr: 0,
            pending_add: None,
            instructions,
        }
    }

    fn render(self) -> String {
        // six rows of 40 plus a newline
        let mut rendered = String::with_capacity(246);
        for signal in self {
            if signal.rendering {
                rendered.push('#');
            } else {
                rendered.push('.');
            }

            if (signal.cycle) % 40 == 0 {
                rendered.push('\n');
            }
        }

        rendered
    }

    fn signal_strength(&self) -> i32 {
        self.register * self.cycle as i32
    }

    fn line_position(&self) -> i32 {
        (self.cycle as i32 - 1) % 40
    }

    fn is_rendering(&self) -> bool {
        self.line_position().abs_diff(self.register) <= 1
    }
}

impl<'i> Iterator for Cycles<'i> {
    type Item = Signal;
    fn next(&mut self) -> Option<Self::Item> {
        if self.cycle > 240 {
            return None;
        }

        let strength = self.signal_strength();
        let rendering = self.is_rendering();

        if let Some(v) = self.pending_add.take() {
            self.register += v;
            self.instruction_ptr += 1;
        } else {
            match self.instructions[self.instruction_ptr] {
                Instruction::Noop => self.instruction_ptr += 1,
                Instruction::Add(v) => self.pending_add = Some(v),
            }
        }

        self.cycle += 1;

        Some(Signal {
            cycle: self.cycle - 1,
            strength,
            rendering,
        })
    }
}

impl<'i> std::fmt::Debug for Cycles<'i> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Cycles")
            .field("cycle", &self.cycle)
            .field("register", &self.register)
            .field("line_position", &self.line_position())
            .field("rendering", &self.is_rendering())
            .field("instruction_ptr", &self.instruction_ptr)
            .field("pending_add", &self.pending_add)
            .field("signal_strength", &self.signal_strength())
            .finish_non_exhaustive()
    }
}

#[cfg(test)]
crate::test_helpers::test_short_input_for_day!(
    10,
    "13140",
    r#"##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....
"#
);
