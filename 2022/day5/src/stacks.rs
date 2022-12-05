#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Stacks {
    pub stacks: Vec<Vec<char>>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Move {
    pub crates_count: usize,
    pub from: usize,
    pub to: usize,
}

impl Stacks {
    pub fn new(stacks_count: usize) -> Self {
        Self {
            stacks: vec![vec![]; stacks_count],
        }
    }

    pub fn push_crates_line(&mut self, line: Vec<Option<char>>) {
        assert!(line.len() <= self.stacks.len());
        self.stacks
            .iter_mut()
            .zip(line.into_iter())
            .for_each(|(stacks, crate_char)| {
                if let Some(ch) = crate_char {
                    stacks.push(ch);
                }
            });
    }

    pub fn push_crates(&mut self, lines: Vec<Vec<Option<char>>>) {
        lines.into_iter().rev().for_each(|line| {
            self.push_crates_line(line);
        });
    }

    pub fn apply_move_9000(&mut self, m: Move) {
        for _ in 0..m.crates_count {
            let crate_char = self.stacks[m.from].pop().unwrap();
            self.stacks[m.to].push(crate_char);
        }
    }

    pub fn apply_move_9001(&mut self, m: Move) {
        let from_old_len = self.stacks[m.from].len();
        let from_new_len = from_old_len - m.crates_count;

        for i in from_new_len..from_old_len {
            let crate_char = self.stacks[m.from][i];
            self.stacks[m.to].push(crate_char);
        }
        self.stacks[m.from].truncate(from_new_len);
    }
}
