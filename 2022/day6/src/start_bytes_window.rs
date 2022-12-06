use itertools::Itertools;
use std::collections::VecDeque;

#[derive(Debug, Default)]
pub struct StartBytesWindow<const N: usize> {
    prev_bytes: VecDeque<u8>,
}

impl<const N: usize> StartBytesWindow<N> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push(&mut self, b: u8) {
        if self.prev_bytes.len() >= N {
            self.prev_bytes.pop_front();
        }
        self.prev_bytes.push_back(b);
    }

    pub fn is_starting_sequence(&self) -> bool {
        (self.prev_bytes.len() == N)
            && self
                .prev_bytes
                .iter()
                .tuple_combinations()
                .all(|(x, y)| x != y)
    }
}

pub type StartOfPacket = StartBytesWindow<4>;
pub type StartOfMessage = StartBytesWindow<14>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn start_chars_window() {
        let mut window = StartOfPacket::new();

        assert_eq!(window.is_starting_sequence(), false);
        window.push(b'C');
        assert_eq!(window.is_starting_sequence(), false);
        window.push(b'A');
        assert_eq!(window.is_starting_sequence(), false);
        window.push(b'Z');
        assert_eq!(window.is_starting_sequence(), false);
        window.push(b'Z');
        assert_eq!(window.is_starting_sequence(), false);
        window.push(b'A');
        assert_eq!(window.is_starting_sequence(), false);
        window.push(b'C');
        assert_eq!(window.is_starting_sequence(), false);
        window.push(b'M');
        assert_eq!(window.is_starting_sequence(), true);
        window.push(b'O');
        assert_eq!(window.is_starting_sequence(), true);
        window.push(b'C');
        assert_eq!(window.is_starting_sequence(), false);
    }
}
