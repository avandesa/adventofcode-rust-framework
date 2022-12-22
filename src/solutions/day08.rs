use crate::PuzzleSolver;

use std::fmt::{Display, Formatter, Result as FmtResult};

pub struct Day08 {
    grid: Grid<u32>,
}

impl PuzzleSolver for Day08 {
    fn with_input(input: String) -> Self {
        let size = input.find('\n').unwrap();
        let contents = input.chars().filter_map(|c| c.to_digit(10)).collect();
        Self {
            grid: Grid::new(size, contents),
        }
    }

    fn part1(&self) -> String {
        let mut visibility = self.grid.mapped(|height| VisibilityMarker {
            height: *height,
            is_visible: false,
        });

        visibility.edit_all_directions(mark_visibility);

        visibility
            .contents
            .iter()
            .filter(|v| v.is_visible)
            .count()
            .to_string()
    }

    fn part2(&self) -> String {
        let mut scores = self.grid.mapped(|height| VisibilityScore {
            height: *height,
            score: 1,
        });

        scores.edit_all_directions(count_score);

        println!("{scores}");
        scores
            .contents
            .iter()
            .map(|s| s.score)
            .max()
            .unwrap()
            .to_string()
    }
}

#[derive(Clone, Copy, Debug)]
struct VisibilityMarker {
    height: u32,
    is_visible: bool,
}

fn mark_visibility(row: &mut [VisibilityMarker]) {
    // Outer trees are always visible
    row[0].is_visible = true;
    let mut max_seen = 0;
    for item in row {
        if max_seen < item.height {
            // There are no trees to the left as tall as this one, so it is visible
            max_seen = item.height;
            item.is_visible = true;
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct VisibilityScore {
    height: u32,
    score: u32,
}

fn count_score(row: &mut [VisibilityScore]) {
    // Ignore outer trees (locks the score at 0 since it's multiplicative)
    row[0].score = 0;

    // Go through every other tree in the row
    for current in 1..row.len() {
        // Starting at the next tree to the right, count how many are visible
        let mut num_visible = 0;
        for other_tree in &row[(current + 1)..] {
            // If the immediate next tree is as tall as this one, then we can only see 1, so we
            // still increment num_visible and break immediately. Otherwise keep counting until we
            // reach the end or another tree at least our height
            num_visible += 1;
            if other_tree.height >= row[current].height {
                break;
            }
        }
        // Multiply the visibility for this view by the current score
        // If this is the last tree in the row, then the loop is never entered so `num_visible`
        // remains 0. This has the same effect as the first line by locking outer trees' scores to 0
        row[current].score *= num_visible as u32;
    }
}

#[derive(Clone, Debug)]
struct Grid<T: Copy + Display> {
    size: usize,
    contents: Vec<T>,
}

impl<T: Copy + Display> Grid<T> {
    /// Construct a new grid. Panics if size^2 does not equal the length of contents
    pub fn new(size: usize, contents: Vec<T>) -> Self {
        assert_eq!(size * size, contents.len());
        Self { size, contents }
    }

    /// Generate a new grid mapping each cell to a new type
    ///
    /// Analogous to `Iterator::map`
    fn mapped<F, O: Copy + Display>(&self, f: F) -> Grid<O>
    where
        F: FnMut(&T) -> O,
    {
        let contents = self.contents.iter().map(f).collect();
        Grid::new(self.size, contents)
    }

    /// Get a mutable slice of the requested row. Panics if row is out of bounds
    fn row_mut(&mut self, row: usize) -> &mut [T] {
        assert!(row < self.size);
        let start = row * self.size;
        let end = (row + 1) * self.size;
        &mut self.contents[start..end]
    }

    /// For each row in the grid, call a function that may edit individual cells
    fn edit_rows<F>(&mut self, edit_fn: &mut F)
    where
        F: FnMut(&mut [T]),
    {
        for row in 0..self.size {
            edit_fn(self.row_mut(row));
        }
    }

    /// Calls the given function on each row in the grid, then rotates the whoel grid
    /// counterclockwise. This is repeated four times. At the end, the grid is in the same
    /// orientation as it was at the start.
    fn edit_all_directions<F>(&mut self, mut edit_fn: F)
    where
        F: FnMut(&mut [T]),
    {
        for _ in 0..4 {
            self.edit_rows(&mut edit_fn);
            self.rotate();
        }
    }

    /// Calculate the index for `contents` for a cell at the given coordinates
    fn i(&self, x: usize, y: usize) -> usize {
        y * self.size + x
    }

    /// Get a copy of the item at the given coordinates
    fn item_at(&self, x: usize, y: usize) -> T {
        self.contents[self.i(x, y)]
    }

    /// Set the value at the given coordinates to the new value
    fn set(&mut self, x: usize, y: usize, val: T) {
        let i = self.i(x, y);
        self.contents[i] = val;
    }

    /// Rotate the whole grid in-place counterclockwise
    ///
    /// Shamelessly stolen from https://afteracademy.com/blog/rotate-matrix/
    pub fn rotate(&mut self) {
        let n = self.size;
        for layer in 0..self.size / 2 {
            let low = layer;
            let high = n - 1 - layer;
            for i in low..high {
                let temp = self.item_at(low, i);
                self.set(low, i, self.item_at(n - 1 - i, low));
                self.set(n - 1 - i, low, self.item_at(n - 1 - low, n - 1 - i));
                self.set(n - 1 - low, n - 1 - i, self.item_at(i, n - 1 - low));
                self.set(i, n - 1 - low, temp);
            }
        }
    }
}

impl<T: Copy + Display> Display for Grid<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        for row in self.contents.chunks_exact(self.size) {
            for item in row {
                write!(f, " {item}")?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

impl Display for VisibilityMarker {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        if self.is_visible { 'Y' } else { 'n' }.fmt(f)
    }
}

impl Display for VisibilityScore {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{:7}", self.score)
    }
}

#[cfg(test)]
crate::test_helpers::test_short_input_for_day!(8, "21", "8");
