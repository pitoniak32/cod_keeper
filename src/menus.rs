use serde::{Deserialize, Serialize};
use strum_macros::{EnumIter, Display};


#[derive(Serialize, Deserialize, Debug, EnumIter, Display, PartialEq, Eq)]
pub enum MainMenuOption {
    EnterGames,
    DisplayStats,
}

#[derive(Serialize, Deserialize, Debug, EnumIter, Display, PartialEq, Eq)]
pub enum DisplayStatsOption {
    Lifetime,
    CurrentStreak,
    Today,
}

#[derive(Serialize, Deserialize, Debug, Clone, EnumIter, Display, PartialEq, Eq)]
pub enum DidWinOption {
    Yes,
    No,
    Quit,
}