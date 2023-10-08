use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumIter};

#[derive(Serialize, Deserialize, Debug, EnumIter, Display, PartialEq, Eq)]
pub enum MainMenuOption {
    EnterGames,
    DisplayStats,
    Quit,
}

#[derive(Serialize, Deserialize, Debug, EnumIter, Display, PartialEq, Eq)]
pub enum DisplayStatsOption {
    Lifetime,
    CurrentStreak,
    Today,
    Maps,
    Back,
}

#[derive(Serialize, Deserialize, Debug, Clone, EnumIter, Display, PartialEq, Eq)]
pub enum DidWinOption {
    Yes,
    No,
    Back,
}
