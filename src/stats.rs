use anyhow::Result;

use std::collections::HashMap;

use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

use crate::{error::Error, map::MapStats, GamePlayed, GunfightMap, DAY_FMT};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct Stats {
    pub lifet: StatsGroup,
    pub today: StatsGroup,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct StatsGroup {
    pub wins: usize,
    pub losses: usize,
    pub high_win_streak: usize,
    pub high_loss_streak: usize,
    pub win_streak: usize,
    pub loss_streak: usize,
    pub last_was_win: bool,
    pub map_stats: HashMap<GunfightMap, MapStats>,
}

impl StatsGroup {
    pub const fn get_all_map_stats(&self) -> &HashMap<GunfightMap, MapStats> {
        &self.map_stats
    }

    pub fn get_map_stats(&self, map: &GunfightMap) -> Option<&MapStats> {
        self.map_stats.get(map)
    }

    pub fn get_win_percentage(&self) -> f32 {
        if self.wins + self.losses == 0 {
            return 0.0;
        }
        (self.wins as f32 / (self.wins + self.losses) as f32) * 100.0
    }
}

impl Stats {
    pub fn new(games: &mut [GamePlayed], today: DateTime<Local>) -> Result<Self, Error> {
        let mut stats = Self {
            lifet: StatsGroup {
                wins: 0,
                losses: 0,
                high_win_streak: 0,
                high_loss_streak: 0,
                win_streak: 0,
                loss_streak: 0,
                last_was_win: true,
                map_stats: HashMap::new(),
            },
            today: StatsGroup {
                wins: 0,
                losses: 0,
                high_win_streak: 0,
                high_loss_streak: 0,
                win_streak: 0,
                loss_streak: 0,
                last_was_win: true,
                map_stats: HashMap::new(),
            },
        };
        let today = today.format(DAY_FMT).to_string();
        let errors = games
            .iter_mut()
            .map(|game| {
                if game.did_win {
                    stats.add_win(game, &today)
                } else {
                    stats.add_loss(game, &today)
                }
            })
            .filter_map(|r| match r {
                Ok(_) => None,
                Err(e) => {
                    eprint!("[ERROR]: {e}");
                    Some(e)
                }
            })
            .collect::<Vec<_>>();

        if !errors.is_empty() {
            return Err(Error::FailedCreatingStats(errors));
        }

        Ok(stats)
    }

    pub fn add_win(&mut self, game: &GamePlayed, today: &str) -> Result<(), Error> {
        if game.date_time.format(DAY_FMT).to_string() == today {
            if self.today.map_stats.contains_key(&game.map) {
                self.today
                    .map_stats
                    .get_mut(&game.map)
                    .ok_or(Error::GunfightMapNotFound(game.map.clone()))?
                    .wins += 1;
            } else {
                self.today
                    .map_stats
                    .insert(game.map.clone(), MapStats { wins: 1, losses: 0 });
            }
            self.today.wins += 1;
            self.today.last_was_win = true;
            if self.today.last_was_win {
                self.today.win_streak += 1;
                if self.today.high_win_streak < self.today.win_streak {
                    self.today.high_win_streak = self.today.win_streak;
                }
                if self.today.high_loss_streak < self.today.loss_streak {
                    self.today.high_loss_streak = self.today.loss_streak;
                }
                self.today.loss_streak = 0;
            }
        }
        if self.lifet.map_stats.contains_key(&game.map) {
            self.lifet
                .map_stats
                .get_mut(&game.map)
                .ok_or(Error::GunfightMapNotFound(game.map.clone()))?
                .wins += 1;
        } else {
            self.lifet
                .map_stats
                .insert(game.map.clone(), MapStats { wins: 1, losses: 0 });
        }
        self.lifet.wins += 1;
        self.lifet.last_was_win = true;
        if self.lifet.last_was_win {
            self.lifet.win_streak += 1;
            if self.lifet.high_win_streak < self.lifet.win_streak {
                self.lifet.high_win_streak = self.lifet.win_streak;
            }
            if self.lifet.high_loss_streak < self.lifet.loss_streak {
                self.lifet.high_loss_streak = self.lifet.loss_streak;
            }
            self.lifet.loss_streak = 0;
        }
        Ok(())
    }

    pub fn add_loss(&mut self, game: &GamePlayed, today: &str) -> Result<(), Error> {
        if game.date_time.format(DAY_FMT).to_string() == today {
            if self.today.map_stats.contains_key(&game.map) {
                self.today
                    .map_stats
                    .get_mut(&game.map)
                    .ok_or(Error::GunfightMapNotFound(game.map.clone()))?
                    .losses += 1;
            } else {
                self.today
                    .map_stats
                    .insert(game.map.clone(), MapStats { wins: 0, losses: 1 });
            }
            self.today.losses += 1;
            self.today.last_was_win = false;
            if !self.today.last_was_win {
                self.today.loss_streak += 1;
                if self.today.high_loss_streak < self.today.loss_streak {
                    self.today.high_loss_streak = self.today.loss_streak;
                }
                if self.today.high_win_streak < self.today.win_streak {
                    self.today.high_win_streak = self.today.win_streak;
                }
                self.today.win_streak = 0;
            }
        }
        if self.lifet.map_stats.contains_key(&game.map) {
            self.lifet
                .map_stats
                .get_mut(&game.map)
                .ok_or(Error::GunfightMapNotFound(game.map.clone()))?
                .losses += 1;
        } else {
            self.lifet
                .map_stats
                .insert(game.map.clone(), MapStats { wins: 0, losses: 1 });
        }
        self.lifet.losses += 1;
        self.lifet.last_was_win = false;
        if !self.lifet.last_was_win {
            self.lifet.loss_streak += 1;
            if self.lifet.high_loss_streak < self.lifet.loss_streak {
                self.lifet.high_loss_streak = self.lifet.loss_streak;
            }
            if self.lifet.high_win_streak < self.lifet.win_streak {
                self.lifet.high_win_streak = self.lifet.win_streak;
            }
            self.lifet.win_streak = 0;
        }
        Ok(())
    }

    pub fn display_map_stats(&self) {
        println!();
        println!("Lifetime:\n---");
        let mut life_map_stats = self
            .lifet
            .get_all_map_stats()
            .iter()
            .collect::<Vec<_>>()
            .clone();

        life_map_stats.sort_by(|a, b| {
            b.1.get_win_percentage()
                .total_cmp(&a.1.get_win_percentage())
        });

        life_map_stats.iter().for_each(|m| {
            println!("{}: {}", m.0, m.1);
        });

        println!();
        println!("Today:\n---");
        let mut today_map_stats = self
            .today
            .get_all_map_stats()
            .iter()
            .collect::<Vec<_>>()
            .clone();

        today_map_stats.sort_by(|a, b| {
            b.1.get_win_percentage()
                .total_cmp(&a.1.get_win_percentage())
        });

        today_map_stats.iter().for_each(|m| {
            println!("{}: {}", m.0, m.1);
        });
        println!();
    }
}

#[cfg(test)]
#[rustfmt::skip]
mod tests {
    use anyhow::Result;

    use chrono::TimeZone;
    use pretty_assertions::assert_eq;

    use crate::GunfightMap;

    use super::*;

    #[test]
    fn test_stats_empty() -> Result<()> {
        // Arrange
        let mut games: Vec<GamePlayed> = Vec::new();

        // Act / Assert
        assert_eq!(
            Stats::new(&mut games, Local::now())?,
            Stats {
                lifet: StatsGroup {wins:0,losses:0,high_win_streak:0,high_loss_streak:0,win_streak:0,loss_streak:0,last_was_win:true, map_stats: HashMap::new() },
                today: StatsGroup { wins: 0, losses: 0, high_win_streak: 0, high_loss_streak: 0,  win_streak: 0, loss_streak: 0, last_was_win: true, map_stats: HashMap::new() },
            },
        );

        Ok(())
    }

    #[test]
    fn test_stats_all_one_not_today() -> Result<()> {
        // Arrange
        let mut games: Vec<GamePlayed> = vec![
            GamePlayed { did_win: false, map: GunfightMap::Asile9, date_time: Local.with_ymd_and_hms(2023, 9, 26, 0, 0, 1).unwrap(), },
            GamePlayed { did_win: true, map: GunfightMap::Asile9, date_time: Local.with_ymd_and_hms(2023, 9, 26, 0, 0, 2).unwrap(), },
        ];
        let mut maps_lifet = HashMap::new();
        maps_lifet.insert(GunfightMap::Asile9, MapStats { wins: 1, losses: 1 });
        let maps_today = HashMap::new();

        // Act / Assert
        assert_eq!(
            Stats::new(
                &mut games,
                Local.with_ymd_and_hms(2023, 9, 29, 0, 0, 0).unwrap()
            )?,
            Stats {
                lifet: StatsGroup { wins: 1, losses: 1, high_win_streak: 1, high_loss_streak: 1,  win_streak: 1, loss_streak: 0, last_was_win: true, map_stats: maps_lifet },
                today: StatsGroup { wins: 0, losses: 0, high_win_streak: 0, high_loss_streak: 0,  win_streak: 0, loss_streak: 0, last_was_win: true, map_stats: maps_today },
            },
        );
        Ok(())
    }

    #[test]
    fn test_stats_add_win()  -> Result<()> {
        // Arrange
        let mut games: Vec<GamePlayed> = vec![
            GamePlayed { did_win: false, map: GunfightMap::Asile9, date_time: Local.with_ymd_and_hms(2023, 9, 26, 0, 0, 1).unwrap(), },
            GamePlayed { did_win: true, map: GunfightMap::Asile9, date_time: Local.with_ymd_and_hms(2023, 9, 26, 0, 0, 2).unwrap(), },
        ];
        let mut maps_lifet = HashMap::new();
        maps_lifet.insert(GunfightMap::Asile9, MapStats { wins: 2, losses: 1 });
        maps_lifet.insert(GunfightMap::Docks, MapStats { wins: 1, losses: 0 });
        let maps_today = HashMap::new();

        // Act
        let mut stats = Stats::new(
            &mut games,
            Local.with_ymd_and_hms(2023, 9, 29, 0, 0, 0).unwrap(),
        )?;
        stats.add_win(
            &GamePlayed { map: GunfightMap::Asile9, did_win: true, date_time: Local.with_ymd_and_hms(2023, 9, 26, 0, 0, 3).unwrap(), },
            "9-29-2023")?;
        stats.add_win(
            &GamePlayed { map: GunfightMap::Docks, did_win: true, date_time: Local.with_ymd_and_hms(2023, 9, 26, 0, 0, 4).unwrap(), },
            "9-29-2023",
        )?;

        // Assert
        assert_eq!(
            stats,
            Stats {
                lifet: StatsGroup { wins: 3, losses: 1, high_win_streak: 3, high_loss_streak: 1, win_streak: 3, loss_streak: 0, last_was_win: true, map_stats: maps_lifet },
                today: StatsGroup { wins: 0, losses: 0, high_win_streak: 0, high_loss_streak: 0, win_streak: 0, loss_streak: 0, last_was_win: true, map_stats: maps_today },
            },
        );
        Ok(())
    }

    #[test]
    fn test_stats_add_loss()  -> Result<(), Error> {
        // Arrange
        let mut games: Vec<GamePlayed> = vec![
            GamePlayed { did_win: false, map: GunfightMap::Asile9, date_time: Local.with_ymd_and_hms(2023, 9, 26, 0, 0, 1).unwrap(), },
            GamePlayed { did_win: true, map: GunfightMap::Asile9, date_time: Local.with_ymd_and_hms(2023, 9, 26, 0, 0, 2).unwrap(), },
            GamePlayed { did_win: true, map: GunfightMap::Asile9, date_time: Local.with_ymd_and_hms(2023, 9, 26, 0, 0, 3).unwrap(), },
        ];
        let mut maps_lifet = HashMap::new();
        maps_lifet.insert(GunfightMap::Asile9, MapStats { wins: 2, losses: 2 });
        maps_lifet.insert(GunfightMap::Docks, MapStats { wins: 0, losses: 1 });
        let maps_today = HashMap::new();

        // Act
        let mut stats = Stats::new(
            &mut games,
            Local.with_ymd_and_hms(2023, 9, 29, 0, 0, 0).unwrap(),
        )?;
        stats.add_loss(
            &GamePlayed { map: GunfightMap::Asile9, did_win: false, date_time: Local.with_ymd_and_hms(2023, 9, 26, 0, 0, 4).unwrap(), },
            "9-29-2023",
        )?;
        stats.add_loss(
            &GamePlayed { map: GunfightMap::Docks, did_win: false, date_time: Local.with_ymd_and_hms(2023, 9, 26, 0, 0, 5).unwrap(), },
            "9-29-2023",
        )?;

        // Assert
        assert_eq!(
            stats,
            Stats {
                lifet: StatsGroup { wins: 2, losses: 3, high_win_streak: 2, high_loss_streak: 2, win_streak: 0, loss_streak: 2, last_was_win: false, map_stats: maps_lifet },
                today: StatsGroup { wins: 0, losses: 0, high_win_streak: 0, high_loss_streak: 0, win_streak: 0, loss_streak: 0, last_was_win: true, map_stats: maps_today },
            },
        );
        Ok(())
    }

    #[test]
    fn test_stats_curr_streak_across_days()  -> Result<(), Error> {
        // Arrange
        let mut games: Vec<GamePlayed> = vec![
            GamePlayed { did_win: false, map: GunfightMap::Asile9, date_time: Local.with_ymd_and_hms(2023, 9, 26, 0, 0, 1).unwrap(), },
            GamePlayed { did_win: true, map: GunfightMap::Asile9, date_time: Local.with_ymd_and_hms(2023, 9, 26, 0, 0, 2).unwrap(), },
            GamePlayed { did_win: true, map: GunfightMap::Asile9, date_time: Local.with_ymd_and_hms(2023, 9, 26, 0, 0, 3).unwrap(), },
            // New day
            GamePlayed { did_win: true, map: GunfightMap::Asile9, date_time: Local.with_ymd_and_hms(2023, 9, 27, 0, 0, 2).unwrap(), },
            GamePlayed { did_win: true, map: GunfightMap::Asile9, date_time: Local.with_ymd_and_hms(2023, 9, 27, 0, 0, 3).unwrap(), },
        ];
        let mut maps_lifet = HashMap::new();
        maps_lifet.insert(GunfightMap::Asile9, MapStats { wins: 5, losses: 1 });
        maps_lifet.insert(GunfightMap::Docks, MapStats { wins: 0, losses: 1 });
        let maps_today = HashMap::new();

        // Act
        let mut stats = Stats::new(
            &mut games,
            Local.with_ymd_and_hms(2023, 9, 29, 0, 0, 0).unwrap(),
        )?;
        stats.add_win(
            &GamePlayed { map: GunfightMap::Asile9, did_win: true, date_time: Local.with_ymd_and_hms(2023, 9, 27, 0, 0, 4).unwrap(), },
            "9-29-2023",
        )?;
        stats.add_loss(
            &GamePlayed { map: GunfightMap::Docks, did_win: false, date_time: Local.with_ymd_and_hms(2023, 9, 27, 0, 0, 5).unwrap(), },
            "9-29-2023",
        )?;

        // Assert
        assert_eq!(
            stats,
            Stats {
                lifet: StatsGroup { wins: 5, losses: 2, high_win_streak: 5, high_loss_streak: 1, win_streak: 0, loss_streak: 1, last_was_win: false, map_stats: maps_lifet },
                today: StatsGroup { wins: 0, losses: 0, high_win_streak: 0, high_loss_streak: 0, win_streak: 0, loss_streak: 0, last_was_win: true, map_stats: maps_today },
            },
        );
        Ok(())
    }

    #[test]
    fn test_stats_all_one_today()  -> Result<(), Error>{
        // Arrange
        let mut games: Vec<GamePlayed> = vec![
            GamePlayed { did_win: false, map: GunfightMap::Asile9, date_time: Local.with_ymd_and_hms(2023, 9, 26, 0, 0, 1).unwrap(), },
            GamePlayed { did_win: true, map: GunfightMap::Asile9, date_time: Local.with_ymd_and_hms(2023, 9, 26, 0, 0, 2).unwrap(), },
            GamePlayed { did_win: true, map: GunfightMap::Asile9, date_time: Local.with_ymd_and_hms(2023, 9, 26, 0, 0, 3).unwrap(), },
            GamePlayed { did_win: false, map: GunfightMap::Asile9, date_time: Local.with_ymd_and_hms(2023, 9, 26, 0, 0, 4).unwrap(), },
            GamePlayed { did_win: false, map: GunfightMap::Asile9, date_time: Local.with_ymd_and_hms(2023, 9, 26, 0, 0, 5).unwrap(), },
            GamePlayed { did_win: true, map: GunfightMap::Asile9, date_time: Local.with_ymd_and_hms(2023, 9, 26, 0, 0, 6).unwrap(), },
            GamePlayed { did_win: true, map: GunfightMap::Asile9, date_time: Local.with_ymd_and_hms(2023, 9, 26, 0, 0, 7).unwrap(), },
            GamePlayed { did_win: true, map: GunfightMap::Asile9, date_time: Local.with_ymd_and_hms(2023, 9, 26, 0, 0, 8).unwrap(), },
            GamePlayed { did_win: true, map: GunfightMap::Asile9, date_time: Local.with_ymd_and_hms(2023, 9, 26, 0, 0, 9).unwrap(), },
            GamePlayed { did_win: false, map: GunfightMap::Asile9, date_time: Local.with_ymd_and_hms(2023, 9, 26, 0, 0, 10).unwrap(), },
            GamePlayed { did_win: true, map: GunfightMap::Asile9, date_time: Local.with_ymd_and_hms(2023, 9, 26, 0, 0, 11).unwrap(), },
            GamePlayed { did_win: true, map: GunfightMap::Asile9, date_time: Local.with_ymd_and_hms(2023, 9, 26, 0, 0, 12).unwrap(), },
            GamePlayed { did_win: true, map: GunfightMap::Asile9, date_time: Local.with_ymd_and_hms(2023, 9, 26, 0, 0, 13).unwrap(), },
            // Different day to test multiday
            GamePlayed { did_win: true, map: GunfightMap::Asile9, date_time: Local.with_ymd_and_hms(2023, 9, 28, 0, 0, 1).unwrap(), },
            GamePlayed { did_win: true, map: GunfightMap::Asile9, date_time: Local.with_ymd_and_hms(2023, 9, 28, 0, 0, 2).unwrap(), },
            GamePlayed { did_win: true, map: GunfightMap::Asile9, date_time: Local.with_ymd_and_hms(2023, 9, 28, 0, 0, 3).unwrap(), },
            GamePlayed { did_win: false, map: GunfightMap::Asile9, date_time: Local.with_ymd_and_hms(2023, 9, 28, 0, 0, 4).unwrap(), },
            GamePlayed { did_win: true, map: GunfightMap::Asile9, date_time: Local.with_ymd_and_hms(2023, 9, 28, 0, 0, 5).unwrap(), },
            GamePlayed { did_win: true, map: GunfightMap::Asile9, date_time: Local.with_ymd_and_hms(2023, 9, 28, 0, 0, 6).unwrap(), },
            GamePlayed { did_win: false, map: GunfightMap::Asile9, date_time: Local.with_ymd_and_hms(2023, 9, 28, 0, 0, 7).unwrap(), },
            GamePlayed { did_win: false, map: GunfightMap::Asile9, date_time: Local.with_ymd_and_hms(2023, 9, 28, 0, 0, 8).unwrap(), },
            GamePlayed { did_win: true, map: GunfightMap::Asile9, date_time: Local.with_ymd_and_hms(2023, 9, 28, 0, 0, 9).unwrap(), },
            GamePlayed { did_win: true, map: GunfightMap::Asile9, date_time: Local.with_ymd_and_hms(2023, 9, 28, 0, 0, 10).unwrap(), },
            GamePlayed { did_win: true, map: GunfightMap::Asile9, date_time: Local.with_ymd_and_hms(2023, 9, 28, 0, 0, 11).unwrap(), },
            GamePlayed { did_win: true, map: GunfightMap::Asile9, date_time: Local.with_ymd_and_hms(2023, 9, 28, 0, 0, 12).unwrap(), },
            GamePlayed { did_win: false, map: GunfightMap::Asile9, date_time: Local.with_ymd_and_hms(2023, 9, 28, 0, 0, 13).unwrap(), },
        ];
        let mut maps_lifet = HashMap::new();
        maps_lifet.insert(GunfightMap::Asile9, MapStats { wins: 18, losses: 8 });
        let mut maps_today = HashMap::new();
        maps_today.insert(GunfightMap::Asile9, MapStats { wins: 9, losses: 4 });

        // Act / Assert
        assert_eq!(
            Stats::new(
                &mut games,
                Local.with_ymd_and_hms(2023, 9, 28, 0, 0, 0).unwrap()
            )?,
            Stats {
                lifet: StatsGroup { wins: 18, losses: 8, high_win_streak: 6, high_loss_streak: 2, win_streak: 0, loss_streak: 1, last_was_win: false, map_stats: maps_lifet },
                today: StatsGroup { wins: 9, losses: 4, high_win_streak: 4, high_loss_streak: 2, win_streak: 0, loss_streak: 1, last_was_win: false, map_stats: maps_today },
            },
        );
        Ok(())
    }

    #[test]
    fn test_stats_get_map()  -> Result<(), Error> {
        // Arrange
        let mut games: Vec<GamePlayed> = vec![
            GamePlayed { did_win: false, map: GunfightMap::Asile9, date_time: Local.with_ymd_and_hms(2023, 9, 26, 0, 0, 1).unwrap(), },
            GamePlayed { did_win: false, map: GunfightMap::Asile9, date_time: Local.with_ymd_and_hms(2023, 9, 26, 0, 0, 2).unwrap(), },
            GamePlayed { did_win: true, map: GunfightMap::Asile9, date_time: Local.with_ymd_and_hms(2023, 9, 27, 0, 0, 2).unwrap(), },
            GamePlayed { did_win: true, map: GunfightMap::Hill, date_time: Local.with_ymd_and_hms(2023, 9, 28, 0, 0, 3).unwrap(), },
            GamePlayed { did_win: false, map: GunfightMap::GulagShowers, date_time: Local.with_ymd_and_hms(2023, 9, 28, 0, 0, 4).unwrap(), },
        ];

        // Act / Assert
        assert_eq!(
            Stats::new(
                &mut games,
                Local.with_ymd_and_hms(2023, 9, 28, 0, 0, 0).unwrap()
            )?.lifet.get_map_stats(&GunfightMap::Asile9),
            Some(&MapStats {
                losses: 2,
                wins: 1,
            }),
        );
        assert_eq!(
            Stats::new(
                &mut games,
                Local.with_ymd_and_hms(2023, 9, 28, 0, 0, 0).unwrap()
            )?.lifet.get_map_stats(&GunfightMap::Hill),
            Some(&MapStats {
                losses: 0,
                wins: 1,
            }),
        );
        assert_eq!(
            Stats::new(
                &mut games,
                Local.with_ymd_and_hms(2023, 9, 28, 0, 0, 0).unwrap()
            )?.lifet.get_map_stats(&GunfightMap::GulagShowers),
            Some(&MapStats {
                losses: 1,
                wins: 0,
            }),
        );
        Ok(())
    }
}
