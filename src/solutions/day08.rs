use crate::PuzzleSolver;

pub struct Day08 {
    grid: Grid,
}

impl PuzzleSolver for Day08 {
    fn with_input(input: String) -> Self {
        let trees = input
            .chars()
            .filter_map(|c| c.to_digit(10))
            .collect::<Vec<_>>();

        let width = input.find('\n').unwrap();
        let height = trees.len() / width;

        let grid = Grid::new(width, height, trees);

        Self { grid }
    }

    fn part1(&self) -> String {
        self.grid.count_visible().to_string()
    }

    fn part2(&self) -> String {
        todo!()
    }
}

fn label_visible(list: &mut [&mut Tree]) {
    let mut max_seen = 0;
    for tree in list.iter_mut() {
        if tree.height > max_seen {
            tree.is_visible = true
        }
        max_seen = u32::max(tree.height, max_seen);
    }
    // Now in reverse
    let mut max_seen = 0;
    for tree in list.iter_mut().rev() {
        if tree.height > max_seen {
            tree.is_visible = true
        }
        max_seen = u32::max(tree.height, max_seen);
    }
}

#[derive(Default, Debug, Clone, Copy)]
struct ScenicScore {
    to_north: Option<u32>,
    to_east: Option<u32>,
    to_south: Option<u32>,
    to_west: Option<u32>,
}

impl ScenicScore {
    fn total(&self) -> u32 {
        self.to_north.expect("north not set")
            * self.to_east.expect("east not set")
            * self.to_south.expect("south not set")
            * self.to_west.expect("west not set")
    }
}

#[derive(Debug, Clone, Copy)]
struct Tree {
    height: u32,
    is_visible: bool,
    scenic_score: ScenicScore,
}

impl Tree {
    fn new(height: u32) -> Self {
        Self {
            height,
            is_visible: false,
            scenic_score: Default::default(),
        }
    }
}
#[derive(Debug)]
struct Grid {
    width: usize,
    height: usize,
    trees: Vec<Tree>,
}

impl Grid {
    fn new(width: usize, height: usize, trees: Vec<u32>) -> Self {
        assert_eq!(width * height, trees.len());
        let trees = trees.into_iter().map(|height| Tree::new(height)).collect();

        let mut grid = Self {
            width,
            height,
            trees,
        };
        grid.mark_visibility();

        grid
    }

    fn mark_visibility(&mut self) {
        // Mark the top and bottom rows as visible
        self.row_iter_mut(0).for_each(|tree| tree.is_visible = true);
        self.row_iter_mut(self.height - 1)
            .for_each(|tree| tree.is_visible = true);

        // Go row-by-row, skipping the top and bottom rows
        for row_num in 1..(self.height - 1) {
            let mut row = self.row_iter_mut(row_num).collect::<Vec<_>>();
            label_visible(&mut row);
        }

        // Mark the left and right columns as visible
        // left
        self.column_iter_mut(0)
            .for_each(|tree| tree.is_visible = true);
        self.column_iter_mut(self.width - 1)
            .for_each(|tree| tree.is_visible = true);

        // Go column-by-column
        for col_num in 1..(self.width - 1) {
            let mut col = self.column_iter_mut(col_num).collect::<Vec<_>>();
            label_visible(&mut col);
        }
    }

    fn row_iter_mut(&mut self, row: usize) -> impl Iterator<Item = &mut Tree> {
        assert!(row < self.height);
        self.trees
            .iter_mut()
            .skip(row * self.width)
            .take(self.width)
    }

    // Get a mutable iterator over all items in the column
    fn column_iter_mut(&mut self, col: usize) -> impl Iterator<Item = &mut Tree> {
        assert!(col < self.width);
        self.trees.iter_mut().skip(col).step_by(self.width)
    }

    fn count_visible(&self) -> usize {
        self.trees.iter().filter(|tree| tree.is_visible).count()
    }

    fn tree_at(&self, x: usize, y: usize) -> Tree {
        assert!(x < self.width, "invalid x coordinate {x}");
        assert!(y < self.height, "invalid y coordinate {y}");

        let idx = x + y * self.width;

        assert!(idx < self.trees.len(), "computed index {idx} out of bounds");

        self.trees[idx]
    }

    fn tree_at_mut(&mut self, x: usize, y: usize) -> &mut Tree {
        assert!(x < self.width, "invalid x coordinate {x}");
        assert!(y < self.height, "invalid y coordinate {y}");

        let idx = x + y * self.width;

        assert!(idx < self.trees.len(), "computed index {idx} out of bounds");

        &mut self.trees[idx]
    }
}

impl std::fmt::Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                let to_write = if self.tree_at(x, y).is_visible {
                    'Y'
                } else {
                    'n'
                };
                write!(f, "{}", to_write)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[cfg(test)]
crate::test_helpers::test_short_input_for_day!(8, "21");
