#[derive(Default, Copy, Clone, PartialEq, Eq, Debug)]
pub enum Move {
    #[default]
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

#[derive(Default, Copy, Clone, PartialEq, Eq, Debug)]
pub enum RoundResult {
    #[default]
    Win = 6,
    Draw = 3,
    Lose = 0,
}

#[derive(Default, Copy, Clone, PartialEq, Eq, Debug)]
pub struct GameRound {
    own_move: Move,
    opponent_move: Move,
}

impl TryFrom<char> for Move {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'A' | 'X' => Ok(Move::Rock),
            'B' | 'Y' => Ok(Move::Paper),
            'C' | 'Z' => Ok(Move::Scissors),
            _ => Err(format!("Trying to convert from '{value}' to janken::Move")),
        }
    }
}

impl TryFrom<char> for RoundResult {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'X' => Ok(RoundResult::Lose),
            'Y' => Ok(RoundResult::Draw),
            'Z' => Ok(RoundResult::Win),
            _ => Err(format!(
                "Trying to convert from '{value}' to janken::RoundResult"
            )),
        }
    }
}

impl TryFrom<(char, char)> for GameRound {
    type Error = String;

    fn try_from(value: (char, char)) -> Result<Self, Self::Error> {
        let own_move = Move::try_from(value.1)?;
        let opponent_move = Move::try_from(value.0)?;

        Ok(GameRound {
            own_move,
            opponent_move,
        })
    }
}

impl Move {
    pub fn score(&self) -> u32 {
        *self as u32
    }

    pub fn beats(&self, opponent: Self) -> bool {
        *self == Self::win_against(opponent)
    }

    pub fn win_against(opponent: Self) -> Self {
        match opponent {
            Move::Rock => Move::Paper,
            Move::Paper => Move::Scissors,
            Move::Scissors => Move::Rock,
        }
    }

    pub fn lose_against(opponent: Self) -> Self {
        match opponent {
            Move::Rock => Move::Scissors,
            Move::Paper => Move::Rock,
            Move::Scissors => Move::Paper,
        }
    }
}

impl RoundResult {
    pub fn score(&self) -> u32 {
        *self as u32
    }
}

impl GameRound {
    pub fn with_result(opponent_move: Move, result: RoundResult) -> Self {
        if result == RoundResult::Draw {
            Self {
                own_move: opponent_move,
                opponent_move,
            }
        } else if result == RoundResult::Win {
            Self {
                own_move: Move::win_against(opponent_move),
                opponent_move,
            }
        } else {
            Self {
                own_move: Move::lose_against(opponent_move),
                opponent_move,
            }
        }
    }

    pub fn score(&self) -> u32 {
        self.own_move.score() + self.result().score()
    }

    pub fn result(&self) -> RoundResult {
        if self.own_move == self.opponent_move {
            RoundResult::Draw
        } else if self.own_move.beats(self.opponent_move) {
            RoundResult::Win
        } else {
            RoundResult::Lose
        }
    }
}
