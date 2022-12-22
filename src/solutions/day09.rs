use crate::PuzzleSolver;
use std::collections::HashSet;

use claims::{assert_ge, assert_le};

pub struct Day09 {
    motions: Vec<Direction>,
}

impl PuzzleSolver for Day09 {
    fn with_input(input: String) -> Self {
        let motions = input
            .lines()
            .flat_map(|line| Motion::from_line(line).flatten())
            .collect();
        Self { motions }
    }

    fn part1(&self) -> String {
        let mut snake = Node::new(2);
        snake.apply_motions(&self.motions);
        snake.history_unique().len().to_string()
    }

    fn part2(&self) -> String {
        let mut snake = Node::new(10);
        snake.apply_motions(&self.motions);
        snake.history_unique().len().to_string()
    }
}

#[derive(Clone, Copy, Debug)]
enum Direction {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

impl Direction {
    fn from_char(c: char) -> Self {
        match c {
            'U' => Self::North,
            'D' => Self::South,
            'L' => Self::West,
            'R' => Self::East,
            _ => panic!("invalid character: {c}"),
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct Motion {
    direction: Direction,
    distance: usize,
}

impl Motion {
    fn from_line(line: &str) -> Self {
        let (direction, distance) = line.split_once(' ').unwrap();
        let direction = Direction::from_char(direction.chars().next().unwrap());
        let distance = distance.parse().unwrap();
        Self {
            direction,
            distance,
        }
    }

    fn flatten(self) -> impl Iterator<Item = Direction> {
        std::iter::repeat(self.direction).take(self.distance)
    }
}

#[derive(Debug, Clone)]
struct Node {
    x: isize,
    y: isize,
    next: Next,
}

#[derive(Debug, Clone)]
enum Next {
    Node { next: Box<Node> },
    Tail { history: Vec<(isize, isize)> },
}

impl Node {
    fn new(length: usize) -> Self {
        let next = if length > 1 {
            Next::Node {
                next: Box::new(Node::new(length - 1)),
            }
        } else {
            Next::Tail {
                history: vec![(0, 0)],
            }
        };
        Self { x: 0, y: 0, next }
    }

    fn apply_motions(&mut self, motions: &[Direction]) {
        for direction in motions {
            self.apply_motion(*direction);
        }
    }

    fn apply_motion(&mut self, direction: Direction) {
        // Move the head
        match direction {
            Direction::North => self.y += 1,
            Direction::NorthEast => {
                self.x += 1;
                self.y += 1;
            }
            Direction::East => self.x += 1,
            Direction::SouthEast => {
                self.x += 1;
                self.y -= 1;
            }
            Direction::South => self.y -= 1,
            Direction::SouthWest => {
                self.x -= 1;
                self.y -= 1;
            }
            Direction::West => self.x -= 1,
            Direction::NorthWest => {
                self.x -= 1;
                self.y += 1;
            }
        }
        match &mut self.next {
            Next::Node { next } => {
                if let Some(next_move) = next_move((self.x, self.y), (next.x, next.y)) {
                    next.apply_motion(next_move);
                }
            }
            Next::Tail { history } => {
                // This is the tail. Add the new location to the history
                history.push((self.x, self.y));
            }
        }
    }

    fn history(&self) -> &[(isize, isize)] {
        match &self.next {
            Next::Node { next } => next.history(),
            Next::Tail { history } => history,
        }
    }

    fn history_unique(&self) -> HashSet<(isize, isize)> {
        self.history().iter().copied().collect()
    }

    fn max_coords(&self) -> MaxCoords {
        match &self.next {
            Next::Node { next } => next.max_coords().coalesce(self.x, self.y),
            Next::Tail { history } => {
                let max_history = history
                    .iter()
                    .copied()
                    .fold(MaxCoords::default(), |max, (x, y)| max.coalesce(x, y));
                max_history.coalesce(self.x, self.y)
            }
        }
    }

    fn nodes(&self) -> NodeIter {
        NodeIter {
            current: Some(self),
        }
    }
}

#[derive(Default, Debug)]
struct MaxCoords {
    min_x: isize,
    max_x: isize,
    min_y: isize,
    max_y: isize,
}

impl MaxCoords {
    fn coalesce(self, other_x: isize, other_y: isize) -> Self {
        Self {
            min_x: isize::min(self.min_x, other_x),
            max_x: isize::max(self.max_x, other_x),
            min_y: isize::min(self.min_y, other_y),
            max_y: isize::max(self.max_y, other_y),
        }
    }

    fn width(&self) -> isize {
        self.max_x - self.min_x
    }

    fn height(&self) -> isize {
        self.max_y - self.min_y
    }
}

fn next_move(head_pos: (isize, isize), tail_pos: (isize, isize)) -> Option<Direction> {
    let dist_x = head_pos.0 - tail_pos.0;
    let dist_y = head_pos.1 - tail_pos.1;

    assert_le!(dist_x, 2);
    assert_ge!(dist_x, -2);
    assert_le!(dist_y, 2);
    assert_ge!(dist_y, -2);

    let dir = match (dist_x, dist_y) {
        // Cardinal directions
        (0, 2) => Direction::North,
        (0, -2) => Direction::South,
        (2, 0) => Direction::East,
        (-2, 0) => Direction::West,

        // Diagonals
        (1, 2) | (2, 2) | (2, 1) => Direction::NorthEast,
        (1, -2) | (2, -2) | (2, -1) => Direction::SouthEast,
        (-1, 2) | (-2, 2) | (-2, 1) => Direction::NorthWest,
        (-1, -2) | (-2, -2) | (-2, -1) => Direction::SouthWest,

        _ => return None,
    };

    Some(dir)
}

struct NodeIter<'n> {
    current: Option<&'n Node>,
}

impl<'n> Iterator for NodeIter<'n> {
    type Item = &'n Node;
    fn next(&mut self) -> Option<Self::Item> {
        let rtn = self.current;

        self.current = self.current.and_then(|current| match &current.next {
            Next::Node { next } => Some(next.as_ref()),
            _ => None,
        });

        rtn
    }
}

impl std::fmt::Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let max_coords = self.max_coords();
        let width = max_coords.width();
        let height = max_coords.height();

        let history_coords = self.history_unique();
        let node_coords = self
            .nodes()
            .map(|node| (node.x, node.y))
            .collect::<HashSet<_>>();

        for i in 0..width * height {
            let x = i % width + max_coords.min_x;
            let y = max_coords.max_y - i / width - 1;

            if i % width == 0 {
                writeln!(f)?;
            }

            let to_write = if x == 0 && y == 0 {
                's'
            } else if node_coords.contains(&(x, y)) {
                'X'
            } else if history_coords.contains(&(x, y)) {
                '#'
            } else {
                '.'
            };

            write!(f, "{to_write}")?;
        }

        Ok(())
    }
}

#[cfg(test)]
crate::test_helpers::test_short_input_for_day!(9, "88", "36");
