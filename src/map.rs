use inquire::Select;
use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter};

use crate::{error::Error, CodVersion};

#[derive(Serialize, Deserialize, Debug, Clone, EnumIter, Display, PartialEq, Eq, Hash, Default)]
pub enum GunfightMap {
    // Both
    Rust,
    Shipment,
    // MW
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
    Shoothouse,
    Speedball,
    Stack,
    Station,
    Trench,
    VerdanskStadium,
    // MW3
    DasHaus,
    StashHouse,
    Alley,
    Blacksite,
    Exhibit,
    Meat,
    TrainingFacility,
    // Both
    #[default]
    Back,
}

const OPTIONS_MW3: &[GunfightMap] = &[
    GunfightMap::DasHaus,
    GunfightMap::StashHouse,
    GunfightMap::Alley,
    GunfightMap::Blacksite,
    GunfightMap::Exhibit,
    GunfightMap::Meat,
    GunfightMap::TrainingFacility,
];
const OPTIONS_BOTH: &[GunfightMap] = &[GunfightMap::Back, GunfightMap::Rust, GunfightMap::Shipment];

impl GunfightMap {
    pub fn get_map_choice(cod_version: &CodVersion) -> Result<Self, Error> {
        let maps = match cod_version {
            CodVersion::MW => Self::iter().filter(Self::is_mw).collect(),
            CodVersion::MW3 => Self::iter().filter(Self::is_mw3).collect(),
        };
        Ok(Select::new("Which Map?", maps).prompt()?)
    }

    pub fn is_mw(map: &Self) -> bool {
        !OPTIONS_MW3.contains(map)
    }

    pub fn is_mw3(map: &Self) -> bool {
        OPTIONS_MW3.contains(map) || OPTIONS_BOTH.contains(map)
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
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

impl std::fmt::Display for MapStats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} - {} ({:.0} %)",
            self.wins,
            self.losses,
            self.get_win_percentage()
        )
    }
}
