use std::fmt;
use std::str::FromStr;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use crate::database::upload::DatabaseMetaData;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Difficulty {
    Easy,
    Medium,
    Hard,
    Extreme,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Challenge {
    pub prize: String,
    pub dalli_klick: DatabaseMetaData,
}

#[derive(Debug, Error)]
pub enum GameError {
    #[error("Failed to parse difficulty '{0}'")]
    ParseDifficultyError(String),
}

impl FromStr for Difficulty {
    type Err = GameError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() { 
            "easy" | "e" => Ok(Difficulty::Easy),
            "medium" | "m" => Ok(Difficulty::Medium),
            "hard" | "h" => Ok(Difficulty::Hard),
            "extreme" | "x" => Ok(Difficulty::Extreme),
            _ => Err(GameError::ParseDifficultyError(s.to_string())),
        }
    }
}

impl fmt::Display for Difficulty {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let difficulty_str = match self {
            Difficulty::Easy => "Easy",
            Difficulty::Medium => "Medium",
            Difficulty::Hard => "Hard",
            Difficulty::Extreme => "Extreme",
        };
        write!(f, "{}", difficulty_str)
    }
}