use std::collections::HashSet;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Move {
    dir: Direction,
    step_count: u8,
}

#[derive(Default, Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Pos(i16, i16);

#[derive(Default, Clone, Copy, PartialEq, Eq, Debug)]
pub struct Knot {
    pos: Pos,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Rope<const N: usize> {
    knots: [Knot; N],
    visited_by_tail: HashSet<Pos>,
}

fn signum(value: i16) -> i16 {
    if value < 0 {
        -1
    } else if value > 0 {
        1
    } else {
        0
    }
}

impl TryFrom<char> for Direction {
    type Error = String;

    fn try_from(value: char) -> Result<Direction, String> {
        match value {
            'U' => Ok(Direction::Up),
            'D' => Ok(Direction::Down),
            'L' => Ok(Direction::Left),
            'R' => Ok(Direction::Right),
            _ => Err(format!(
                "Tried to convert invalid char '{value}' to Movement"
            )),
        }
    }
}

impl From<Direction> for Pos {
    fn from(value: Direction) -> Self {
        match value {
            Direction::Up => Self::new(0, 1),
            Direction::Down => Self::new(0, -1),
            Direction::Left => Self::new(-1, 0),
            Direction::Right => Self::new(1, 0),
        }
    }
}

impl std::ops::Add<Self> for Pos {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl std::ops::Sub<Self> for Pos {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl std::ops::AddAssign<Self> for Pos {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
    }
}

impl std::ops::SubAssign<Self> for Pos {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
        self.1 -= rhs.1;
    }
}

impl Pos {
    pub fn new(x: i16, y: i16) -> Self {
        Self(x, y)
    }

    pub fn single_step(&self) -> Self {
        Self(signum(self.0), signum(self.1))
    }
}

impl Move {
    pub fn new(dir: Direction, step_count: u8) -> Self {
        Self { dir, step_count }
    }
}

impl Knot {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn apply(&mut self, m: Direction) {
        self.pos += m.into()
    }

    pub fn follow(&mut self, parent_pos: Pos) {
        let diff = parent_pos - self.pos;
        if diff.0.abs() >= 2 || diff.1.abs() >= 2 {
            if diff.0 == 0 {
                self.pos += Pos(0, signum(diff.1));
            } else if diff.1 == 0 {
                self.pos += Pos(signum(diff.0), 0);
            } else {
                self.pos += Pos(signum(diff.0), signum(diff.1));
            }
        }
    }
}

impl<const N: usize> Default for Rope<N> {
    fn default() -> Self {
        let mut rope = Self {
            knots: [Knot::default(); N],
            visited_by_tail: HashSet::<Pos>::default(),
        };
        rope.visited_by_tail.insert(Pos::default());
        rope
    }
}

impl<const N: usize> Rope<N> {
    pub fn new() -> Self {
        Self::default()
    }

    fn apply_step(&mut self, dir: Direction) {
        self.knots[0].apply(dir);

        for i_child in 1..self.knots.len() {
            self.knots[i_child].follow(self.knots[i_child - 1].pos);
        }

        self.visited_by_tail.insert(self.knots.last().unwrap().pos);

        //println!("{dir:?}\t{:?}\t{:?}", self.knots.first().unwrap().pos, self.knots.last().unwrap().pos);
    }

    pub fn apply_move(&mut self, m: Move) {
        for _ in 0..m.step_count {
            self.apply_step(m.dir);
        }
    }

    pub fn apply_moves(&mut self, moves: &[Move]) {
        for &m in moves.iter() {
            self.apply_move(m);
        }
    }

    pub fn visited_by_tail_count(&self) -> usize {
        self.visited_by_tail.len()
    }
}
