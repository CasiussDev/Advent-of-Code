pub const START_POS_KEY: u8 = 0xFF;
pub const END_POS_KEY: u8 = 0xFE;
const START_HEIGHT: u8 = 0;
const END_HEIGHT: u8 = b'z' - b'a';

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct DirectionIterator {
    next_dir: Option<Direction>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
pub struct Coords {
    x: usize,
    y: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Heightmap {
    elevations: Vec<Vec<u8>>,
    start: Coords,
    end: Coords,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Path {
    steps: Vec<Coords>,
}

impl Direction {
    pub fn iter() -> DirectionIterator {
        DirectionIterator {
            next_dir: Some(Direction::Up),
        }
    }
}

impl Iterator for DirectionIterator {
    type Item = Direction;

    fn next(&mut self) -> Option<Self::Item> {
        let next_dir = self.next_dir;

        match self.next_dir {
            Some(Direction::Up) => Some(Direction::Down),
            Some(Direction::Down) => Some(Direction::Left),
            Some(Direction::Left) => Some(Direction::Right),
            Some(Direction::Right) => None,
            None => None,
        };

        next_dir
    }
}

impl Coords {
    pub fn apply_dir(&self, dir: Direction) -> Self {
        match dir {
            Direction::Up => Self {
                x: self.x,
                y: self.y + 1,
            },
            Direction::Up => Self {
                x: self.x,
                y: self.y - 1,
            },
            Direction::Left => Self {
                x: self.x - 1,
                y: self.y,
            },
            Direction::Right => Self {
                x: self.x + 1,
                y: self.y,
            },
        }
    }
}

impl Path {
    pub fn step_not_visited(&self, step_to_add: Coords) -> bool {
        let path_found = self.steps.iter().position(|&step| step == step_to_add);
        if let None = path_found {
            true
        } else {
            false
        }
    }

    pub fn add_step(&mut self, step_to_add: Coords) {
        assert!(self.step_not_visited(step_to_add));

        self.steps.push(step_to_add);
    }

    pub fn remove_last_step(&mut self) {
        self.steps.pop();
        assert!(!self.steps.is_empty());
    }

    pub fn len(&self) -> usize {
        self.steps.len()
    }
}

impl Heightmap {
    pub fn new(elevations: Vec<Vec<u8>>) -> Self {
        let mut heightmap = Self {
            elevations,
            start: Default::default(),
            end: Default::default(),
        };

        let mut rows_iter = heightmap.elevations.iter();
        let x_dim = rows_iter.next().unwrap().len();
        for row in rows_iter {
            assert_eq!(x_dim, row.len());
        }

        let start_pos_index = heightmap
            .elevations
            .iter()
            .flatten()
            .position(|&h| h == START_POS_KEY)
            .unwrap();
        heightmap.start = heightmap.index_to_coords(start_pos_index);
        heightmap.elevations[heightmap.start.y][heightmap.start.x] = START_HEIGHT;

        let end_pos_index = heightmap
            .elevations
            .iter()
            .flatten()
            .position(|&h| h == END_POS_KEY)
            .unwrap();
        heightmap.end = heightmap.index_to_coords(end_pos_index);
        heightmap.elevations[heightmap.end.y][heightmap.end.x] = END_HEIGHT;

        heightmap
    }

    fn index_to_coords(&self, index: usize) -> Coords {
        let x_dim = self.elevations[0].len();
        Coords {
            x: index % x_dim,
            y: index / x_dim,
        }
    }

    pub fn new_path(&self) -> Path {
        Path {
            steps: vec![self.start],
        }
    }

    pub fn can_transit_to_pos(&self, new_step: Coords, path: Path) -> bool {
        let last_step = path.steps.last().unwrap();
        let current_height = self.elevations[last_step.y][last_step.x];
        let new_height = self.elevations[new_step.y][new_step.x];
        (new_height + 1) <= current_height
    }

    pub fn is_finished(&self, path: Path) -> bool {
        self.end == *path.steps.last().unwrap()
    }
}
