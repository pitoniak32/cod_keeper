use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumIter};

#[derive(Serialize, Deserialize, Debug, EnumIter, Display, PartialEq, Eq, Default)]
pub enum MainMenuOption {
    EnterGames,
    DisplayStats,
    #[default]
    Back,
}

#[derive(Serialize, Deserialize, Debug, EnumIter, Display, PartialEq, Eq, Default)]
pub enum DisplayStatsOption {
    Lifetime,
    CurrentStreak,
    Today,
    Maps,
    OneMap,
    #[default]
    Back,
}

#[derive(Serialize, Deserialize, Debug, Clone, EnumIter, Display, PartialEq, Eq, Default)]
pub enum DidWinOption {
    Yes,
    No,
    #[default]
    Back,
}
