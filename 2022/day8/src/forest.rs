#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Forest {
    trees: Vec<Vec<u8>>,
}

impl Forest {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_row(&mut self, row: Vec<u8>) {
        if !self.trees.is_empty() {
            assert_eq!(self.trees[0].len(), row.len());
        }
        self.trees.push(row);
    }

    pub fn is_visible(&self, coords: (usize, usize)) -> bool {
        self.is_visible_up(coords)
            || self.is_visible_down(coords)
            || self.is_visible_left(coords)
            || self.is_visible_right(coords)
    }

    pub fn is_visible_up(&self, coords: (usize, usize)) -> bool {
        let compare_to = self.trees[coords.1][coords.0];
        (0..(coords.1))
            .map(|y| self.trees[y][coords.0])
            .all(|h| h < compare_to)
    }

    pub fn view_distance_up(&self, coords: (usize, usize)) -> usize {
        let compare_to = self.trees[coords.1][coords.0];
        let block = (0..(coords.1))
            .rev()
            .find(|y| self.trees[*y][coords.0] >= compare_to);

        if let Some(y) = block {
            coords.1 - y
        } else {
            coords.1
        }
    }

    pub fn is_visible_down(&self, coords: (usize, usize)) -> bool {
        let compare_to = self.trees[coords.1][coords.0];
        ((coords.1 + 1)..self.trees.len())
            .map(|y| self.trees[y][coords.0])
            .all(|h| h < compare_to)
    }

    pub fn view_distance_down(&self, coords: (usize, usize)) -> usize {
        let compare_to = self.trees[coords.1][coords.0];
        let block =
            ((coords.1 + 1)..self.trees.len()).find(|y| self.trees[*y][coords.0] >= compare_to);

        if let Some(y) = block {
            y - coords.1
        } else {
            self.trees.len() - coords.1 - 1
        }
    }

    pub fn is_visible_left(&self, coords: (usize, usize)) -> bool {
        let compare_to = self.trees[coords.1][coords.0];
        (0..(coords.0))
            .map(|x| self.trees[coords.1][x])
            .all(|h| h < compare_to)
    }

    pub fn view_distance_left(&self, coords: (usize, usize)) -> usize {
        let compare_to = self.trees[coords.1][coords.0];
        let block = (0..(coords.0))
            .rev()
            .find(|x| self.trees[coords.1][*x] >= compare_to);

        if let Some(x) = block {
            coords.0 - x
        } else {
            coords.0
        }
    }

    pub fn is_visible_right(&self, coords: (usize, usize)) -> bool {
        let compare_to = self.trees[coords.1][coords.0];
        ((coords.0 + 1)..self.trees.len())
            .map(|x| self.trees[coords.1][x])
            .all(|h| h < compare_to)
    }

    pub fn view_distance_right(&self, coords: (usize, usize)) -> usize {
        let compare_to = self.trees[coords.1][coords.0];

        let block =
            ((coords.0 + 1)..self.trees[0].len()).find(|x| self.trees[coords.1][*x] >= compare_to);

        if let Some(x) = block {
            x - coords.0
        } else {
            self.trees[0].len() - coords.0 - 1
        }
    }

    pub fn visible_count(&self) -> usize {
        let trees_per_row = self.trees[0].len();
        let tree_count = self.trees.len() * self.trees[0].len();
        (0..tree_count)
            .map(|i| (i % trees_per_row, i / trees_per_row))
            .filter(|coords| self.is_visible(*coords))
            .count()
    }

    pub fn scenic_score(&self, coords: (usize, usize)) -> usize {
        self.view_distance_up(coords)
            * self.view_distance_down(coords)
            * self.view_distance_left(coords)
            * self.view_distance_right(coords)
    }

    pub fn max_scenic_score(&self) -> usize {
        let trees_per_row = self.trees[0].len();
        let tree_count = self.trees.len() * self.trees[0].len();
        (0..tree_count)
            .map(|i| (i % trees_per_row, i / trees_per_row))
            .map(|coords| self.scenic_score(coords))
            .max()
            .unwrap()
    }
}
