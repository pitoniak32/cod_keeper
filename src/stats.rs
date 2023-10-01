use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

use crate::{GamePlayed, DAY_FMT};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Stats {
    pub lifet: StatsGroup,
    pub today: StatsGroup,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct StatsGroup {
    pub wins: usize,
    pub losses: usize,
    pub win_streak: usize,
    pub loss_streak: usize,
    pub high_win_streak: usize,
    pub high_loss_streak: usize,
    pub last_was_win: bool,
}

impl Stats {
    pub fn new(games: &mut Vec<GamePlayed>, today: DateTime<Local>) -> Stats {
        let mut stats = Stats {
            lifet: StatsGroup {
                wins: 0,
                losses: 0,
                win_streak: 0,
                loss_streak: 0,
                high_win_streak: 0,
                high_loss_streak: 0,
                last_was_win: true,
            },
            today: StatsGroup {
                wins: 0,
                losses: 0,
                win_streak: 0,
                loss_streak: 0,
                high_win_streak: 0,
                high_loss_streak: 0,
                last_was_win: true,
            },
        };
        let today = today.format(DAY_FMT).to_string();
        games.iter_mut().for_each(|game| {
            if game.did_win {
                stats.add_win(game, today.clone());
            } else {
                stats.add_loss(game, today.clone());
            }
        });

        stats
    }

    pub fn add_win(&mut self, game: &mut GamePlayed, today: String) {
        if game.date_time.format(DAY_FMT).to_string() == today {
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
    }

    pub fn add_loss(&mut self, game: &mut GamePlayed, today: String) {
        if game.date_time.format(DAY_FMT).to_string() == today {
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
    }
}

#[cfg(test)]
mod tests {
    use chrono::TimeZone;

    use crate::GunfightMap;

    use super::*;

    #[test]
    fn test_stats_empty() {
        let mut games: Vec<GamePlayed> = Vec::new();
        assert_eq!(
            Stats::new(&mut games, Local::now()),
            Stats {
                lifet: StatsGroup {
                    wins: 0,
                    losses: 0,
                    win_streak: 0,
                    loss_streak: 0,
                    high_win_streak: 0,
                    high_loss_streak: 0,
                    last_was_win: true,
                },
                today: StatsGroup {
                    wins: 0,
                    losses: 0,
                    win_streak: 0,
                    loss_streak: 0,
                    high_win_streak: 0,
                    high_loss_streak: 0,
                    last_was_win: true,
                },
            },
        );
    }

    #[test]
    fn test_stats_all_one_not_today() {
        let mut games: Vec<GamePlayed> = vec![
            GamePlayed {
                did_win: false,
                map: GunfightMap::Asile9,
                date_time: Local::now(),
            },
            GamePlayed {
                did_win: true,
                map: GunfightMap::Asile9,
                date_time: Local::now(),
            },
        ];
        assert_eq!(
            Stats::new(
                &mut games,
                Local.with_ymd_and_hms(2023, 09, 29, 0, 0, 0).unwrap()
            ),
            Stats {
                lifet: StatsGroup {
                    wins: 1,
                    losses: 1,
                    win_streak: 1,
                    loss_streak: 1,
                    high_win_streak: 1,
                    high_loss_streak: 1,
                    last_was_win: true,
                },
                today: StatsGroup {
                    wins: 0,
                    losses: 0,
                    win_streak: 0,
                    loss_streak: 0,
                    high_win_streak: 0,
                    high_loss_streak: 0,
                    last_was_win: true,
                },
            },
        );
    }

    #[test]
    fn test_stats_all_one_today() {
        let mut games: Vec<GamePlayed> = vec![
            GamePlayed {
                did_win: false,
                map: GunfightMap::Asile9,
                date_time: Local.with_ymd_and_hms(2023, 09, 26, 0, 1, 1).unwrap(),
            },
            GamePlayed {
                did_win: true,
                map: GunfightMap::Asile9,
                date_time: Local.with_ymd_and_hms(2023, 09, 26, 0, 2, 2).unwrap(),
            },
            GamePlayed {
                did_win: true,
                map: GunfightMap::Asile9,
                date_time: Local.with_ymd_and_hms(2023, 09, 26, 0, 3, 3).unwrap(),
            },
            GamePlayed {
                did_win: false,
                map: GunfightMap::Asile9,
                date_time: Local.with_ymd_and_hms(2023, 09, 26, 0, 4, 4).unwrap(),
            },
            GamePlayed {
                did_win: false,
                map: GunfightMap::Asile9,
                date_time: Local.with_ymd_and_hms(2023, 09, 26, 0, 5, 5).unwrap(),
            },
            GamePlayed {
                did_win: true,
                map: GunfightMap::Asile9,
                date_time: Local.with_ymd_and_hms(2023, 09, 26, 0, 6, 6).unwrap(),
            },
            GamePlayed {
                did_win: true,
                map: GunfightMap::Asile9,
                date_time: Local.with_ymd_and_hms(2023, 09, 26, 0, 7, 7).unwrap(),
            },
            GamePlayed {
                did_win: true,
                map: GunfightMap::Asile9,
                date_time: Local.with_ymd_and_hms(2023, 09, 26, 0, 8, 8).unwrap(),
            },
            GamePlayed {
                did_win: true,
                map: GunfightMap::Asile9,
                date_time: Local.with_ymd_and_hms(2023, 09, 26, 0, 9, 9).unwrap(),
            },
            GamePlayed {
                did_win: false,
                map: GunfightMap::Asile9,
                date_time: Local.with_ymd_and_hms(2023, 09, 26, 0, 10, 10).unwrap(),
            },
            GamePlayed {
                did_win: true,
                map: GunfightMap::Asile9,
                date_time: Local.with_ymd_and_hms(2023, 09, 26, 0, 11, 11).unwrap(),
            },
            GamePlayed {
                did_win: true,
                map: GunfightMap::Asile9,
                date_time: Local.with_ymd_and_hms(2023, 09, 26, 0, 12, 12).unwrap(),
            },
            GamePlayed {
                did_win: true,
                map: GunfightMap::Asile9,
                date_time: Local.with_ymd_and_hms(2023, 09, 26, 0, 13, 13).unwrap(),
            },
            // Different day to test multiday
            GamePlayed {
                did_win: true,
                map: GunfightMap::Asile9,
                date_time: Local.with_ymd_and_hms(2023, 09, 28, 0, 1, 1).unwrap(),
            },
            GamePlayed {
                did_win: true,
                map: GunfightMap::Asile9,
                date_time: Local.with_ymd_and_hms(2023, 09, 28, 0, 2, 2).unwrap(),
            },
            GamePlayed {
                did_win: true,
                map: GunfightMap::Asile9,
                date_time: Local.with_ymd_and_hms(2023, 09, 28, 0, 3, 3).unwrap(),
            },
            GamePlayed {
                did_win: false,
                map: GunfightMap::Asile9,
                date_time: Local.with_ymd_and_hms(2023, 09, 28, 0, 4, 4).unwrap(),
            },
            GamePlayed {
                did_win: true,
                map: GunfightMap::Asile9,
                date_time: Local.with_ymd_and_hms(2023, 09, 28, 0, 5, 5).unwrap(),
            },
            GamePlayed {
                did_win: true,
                map: GunfightMap::Asile9,
                date_time: Local.with_ymd_and_hms(2023, 09, 28, 0, 6, 6).unwrap(),
            },
            GamePlayed {
                did_win: false,
                map: GunfightMap::Asile9,
                date_time: Local.with_ymd_and_hms(2023, 09, 28, 0, 7, 7).unwrap(),
            },
            GamePlayed {
                did_win: false,
                map: GunfightMap::Asile9,
                date_time: Local.with_ymd_and_hms(2023, 09, 28, 0, 8, 8).unwrap(),
            },
            GamePlayed {
                did_win: true,
                map: GunfightMap::Asile9,
                date_time: Local.with_ymd_and_hms(2023, 09, 28, 0, 9, 9).unwrap(),
            },
            GamePlayed {
                did_win: true,
                map: GunfightMap::Asile9,
                date_time: Local.with_ymd_and_hms(2023, 09, 28, 0, 10, 10).unwrap(),
            },
            GamePlayed {
                did_win: true,
                map: GunfightMap::Asile9,
                date_time: Local.with_ymd_and_hms(2023, 09, 28, 0, 11, 11).unwrap(),
            },
            GamePlayed {
                did_win: true,
                map: GunfightMap::Asile9,
                date_time: Local.with_ymd_and_hms(2023, 09, 28, 0, 12, 12).unwrap(),
            },
            GamePlayed {
                did_win: false,
                map: GunfightMap::Asile9,
                date_time: Local.with_ymd_and_hms(2023, 09, 28, 0, 13, 13).unwrap(),
            },
        ];
        assert_eq!(
            Stats::new(
                &mut games,
                Local.with_ymd_and_hms(2023, 09, 26, 0, 0, 0).unwrap()
            ),
            Stats {
                lifet: StatsGroup {
                    wins: 18,
                    losses: 8,
                    win_streak: 6,
                    loss_streak: 2,
                    high_win_streak: 6,
                    high_loss_streak: 2,
                    last_was_win: true,
                },
                today: StatsGroup {
                    wins: 9,
                    losses: 4,
                    win_streak: 4,
                    loss_streak: 2,
                    high_win_streak: 4,
                    high_loss_streak: 2,
                    last_was_win: true,
                },
            },
        );
    }
}
