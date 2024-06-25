use std::fmt;

use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumIter};

#[derive(Serialize, Deserialize, Debug, Clone, EnumIter, Display, PartialEq, Eq, Hash, Default)]
pub enum GunfightMap {
    #[default]
    Back,
    Asile9,
    Atrium,
    Bazaar,
    Cargo,
    Docks,
    Drainage,
    GulagShowers,
    Hill,
    King,
    Livestock,
    Pine,
    Rust,
    Shipment,
    Shoothouse,
    Speedball,
    Stack,
    Station,
    Trench,
    VerdanskStadium,
}

// impl fmt::Display for GunfightMap {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//       write!(f, "{}", self.to_string())
//     }
// }

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct MapStats {
    pub wins: usize,
    pub losses: usize,
}

impl MapStats {
    pub fn get_win_percentage(&self) -> f32 {
        if self.wins + self.losses == 0 {
            return 0.0;
        }
        (self.wins as f32 / (self.wins + self.losses) as f32) * 100.0
    }
}

impl fmt::Display for MapStats {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} - {} ({:.0} %)",
            self.wins,
            self.losses,
            self.get_win_percentage()
        )
    }
}
