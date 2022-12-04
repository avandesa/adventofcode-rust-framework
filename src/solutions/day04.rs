use crate::PuzzleSolver;

pub struct Day04 {
    pairs: Vec<Pair>,
}

impl PuzzleSolver for Day04 {
    fn with_input(input: String) -> Self {
        let pairs = input.lines().map(Pair::from_line).collect();

        Self { pairs }
    }

    fn part1(&self) -> String {
        self.pairs
            .iter()
            .filter(|pair| pair.one_contains_other())
            .count()
            .to_string()
    }

    fn part2(&self) -> String {
        self.pairs
            .iter()
            .filter(|pair| pair.has_any_overlap())
            .count()
            .to_string()
    }
}

#[derive(Debug, Clone, Copy)]
struct Assigment {
    start: u32,
    end: u32,
}

impl Assigment {
    fn fully_contains(&self, other: Assigment) -> bool {
        self.start <= other.start && self.end >= other.end
    }

    fn overlaps(&self, other: Assigment) -> bool {
        let self_range = self.start..=self.end;
        let other_range = other.start..=other.end;
        self_range.contains(&other.start)
            || self_range.contains(&other.end)
            || other_range.contains(&self.start)
            || other_range.contains(&self.end)
    }
}

#[derive(Debug, Clone, Copy)]
struct Pair(Assigment, Assigment);

impl Pair {
    fn from_line(line: &str) -> Self {
        let re = crate::regex!(r#"^(\d+)-(\d+),(\d+)-(\d+)$"#);
        let captures = re.captures(line).unwrap();

        let a = captures[1].parse().unwrap();
        let b = captures[2].parse().unwrap();
        let c = captures[3].parse().unwrap();
        let d = captures[4].parse().unwrap();

        Self(
            Assigment { start: a, end: b },
            Assigment { start: c, end: d },
        )
    }

    fn one_contains_other(&self) -> bool {
        self.0.fully_contains(self.1) || self.1.fully_contains(self.0)
    }

    fn has_any_overlap(&self) -> bool {
        self.0.overlaps(self.1)
    }
}

#[cfg(test)]
crate::test_helpers::test_short_input_for_day!(4, "2", "4");
