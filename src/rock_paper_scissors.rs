use std::cmp::Ordering;
use std::str::FromStr;
use std::string::ParseError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RockPaperScissors {
    Rock,
    Paper,
    Scissors,
}

impl FromStr for RockPaperScissors {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Self::Rock),
            "B" | "Y" => Ok(Self::Paper),
            "C" | "Z" => Ok(Self::Scissors),
            s => panic!("Unable to convert invalid string \"{s}\" to RockPaperScissors"),
        }
    }
}

impl PartialOrd for RockPaperScissors {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for RockPaperScissors {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Self::Rock, Self::Rock) => Ordering::Equal,
            (Self::Rock, Self::Paper) => Ordering::Less,
            (Self::Rock, Self::Scissors) => Ordering::Greater,
            (Self::Paper, Self::Paper) => Ordering::Equal,
            (Self::Paper, Self::Scissors) => Ordering::Less,
            (Self::Paper, Self::Rock) => Ordering::Greater,
            (Self::Scissors, Self::Scissors) => Ordering::Equal,
            (Self::Scissors, Self::Rock) => Ordering::Less,
            (Self::Scissors, Self::Paper) => Ordering::Greater,
        }
    }
}

impl RockPaperScissors {
    fn choice_score(&self) -> u32 {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }

    pub fn score(&self, other: &Self) -> u32 {
        let choice_score = self.choice_score();

        let round_score = match self.cmp(other) {
            Ordering::Greater => 6,
            Ordering::Equal => 3,
            Ordering::Less => 0,
        };

        choice_score + round_score
    }

    pub fn get_move_to_be(ordering: Ordering, other: Self) -> Self {
        match (ordering, other) {
            (Ordering::Equal, x) => x,
            (Ordering::Greater, Self::Rock) => Self::Paper,
            (Ordering::Greater, Self::Paper) => Self::Scissors,
            (Ordering::Greater, Self::Scissors) => Self::Rock,
            (Ordering::Less, Self::Rock) => Self::Scissors,
            (Ordering::Less, Self::Paper) => Self::Rock,
            (Ordering::Less, Self::Scissors) => Self::Paper,
        }
    }
}
