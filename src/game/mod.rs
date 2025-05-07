use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Difficulty {
    Easy,
    Medium,
    Hard,
    Extreme,
}

#[derive(Debug)]
pub struct ParseDifficultyError;

impl FromStr for Difficulty {
    type Err = ParseDifficultyError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() { 
            "easy" | "e" => Ok(Difficulty::Easy),
            "medium" | "m" => Ok(Difficulty::Medium),
            "hard" | "h" => Ok(Difficulty::Hard),
            "extreme" | "x" => Ok(Difficulty::Extreme),
            _ => Err(ParseDifficultyError),
        }
    }
}