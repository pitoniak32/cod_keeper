use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

use crate::{GamePlayed, DAY_FMT};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Stats {
    pub lifet_wins: usize,
    pub lifet_losses: usize,
    pub lifet_win_streak: usize,
    pub lifet_loss_streak: usize,
    pub today_wins: usize,
    pub today_losses: usize,
    pub today_win_streak: usize,
    pub today_loss_streak: usize,
    pub current_streak: usize,
    pub is_streak_win: bool,
}

impl Stats {
    pub fn new(games: &mut Vec<GamePlayed>, today: DateTime<Local>) -> Stats {
        let mut total_wins = 0;
        let mut total_losses = 0;
        let mut total_curr_win_streak = 0;
        let mut total_curr_loss_streak = 0;
        let mut total_highest_win_streak = 0;
        let mut total_highest_loss_streak = 0;

        let mut total_last_was_win = false;

        let mut todays_wins = 0;
        let mut todays_losses = 0;
        let mut todays_curr_win_streak = 0;
        let mut todays_curr_loss_streak = 0;
        let mut todays_highest_win_streak = 0;
        let mut todays_highest_loss_streak = 0;

        let mut today_last_was_win = false;

        let today = today.format(DAY_FMT).to_string();
        games.iter_mut().for_each(|game| {
            if game.did_win {
                if game.date_time.format(DAY_FMT).to_string() == today {
                    todays_wins += 1;
                    today_last_was_win = true;
                    if today_last_was_win {
                        todays_curr_win_streak += 1;
                        if todays_highest_win_streak < todays_curr_win_streak {
                            todays_highest_win_streak = todays_curr_win_streak;
                        }
                        if todays_highest_loss_streak < todays_curr_loss_streak {
                            todays_highest_loss_streak = todays_curr_loss_streak;
                        }
                        todays_curr_loss_streak = 0;
                    }
                }
                total_wins += 1;
                total_last_was_win = true;
                if total_last_was_win {
                    total_curr_win_streak += 1;
                    if total_highest_win_streak < total_curr_win_streak {
                        total_highest_win_streak = total_curr_win_streak;
                    }
                    if total_highest_loss_streak < total_curr_loss_streak {
                        total_highest_loss_streak = total_curr_loss_streak;
                    }
                    total_curr_loss_streak = 0;
                }
            } else {
                if game.date_time.format(DAY_FMT).to_string() == today {
                    todays_losses += 1;
                    today_last_was_win = false;
                    if !today_last_was_win {
                        todays_curr_loss_streak += 1;
                        if todays_highest_loss_streak < todays_curr_loss_streak {
                            todays_highest_loss_streak = todays_curr_loss_streak;
                        }
                        if todays_highest_win_streak < todays_curr_win_streak {
                            todays_highest_win_streak = todays_curr_win_streak;
                        }
                        todays_curr_win_streak = 0;
                    }
                }
                total_losses += 1;
                total_last_was_win = false;
                if !total_last_was_win {
                    total_curr_loss_streak += 1;
                    if total_highest_loss_streak < total_curr_loss_streak {
                        total_highest_loss_streak = total_curr_loss_streak;
                    }
                    if total_highest_win_streak < total_curr_win_streak {
                        total_highest_win_streak = total_curr_win_streak;
                    }
                    total_curr_win_streak = 0;
                }
            }
        });

        Stats {
            lifet_wins: total_wins,
            lifet_losses: total_losses,
            lifet_win_streak: total_highest_win_streak,
            lifet_loss_streak: total_highest_loss_streak,
            today_wins: todays_wins,
            today_losses: todays_losses,
            today_win_streak: todays_highest_win_streak,
            today_loss_streak: todays_highest_loss_streak,
            current_streak: 0,
            is_streak_win: true,
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
                lifet_wins: 0,
                lifet_losses: 0,
                lifet_win_streak: 0,
                lifet_loss_streak: 0,
                today_wins: 0,
                today_losses: 0,
                today_win_streak: 0,
                today_loss_streak: 0,
                current_streak: 0,
                is_streak_win: true,
            }
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
                lifet_wins: 1,
                lifet_losses: 1,
                lifet_win_streak: 1,
                lifet_loss_streak: 1,
                today_wins: 0,
                today_losses: 0,
                today_win_streak: 0,
                today_loss_streak: 0,
                current_streak: 0,
                is_streak_win: true,
            }
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
                lifet_wins: 18,
                lifet_losses: 8,
                lifet_win_streak: 6,
                lifet_loss_streak: 2,
                today_wins: 9,
                today_losses: 4,
                today_win_streak: 4,
                today_loss_streak: 2,
                current_streak: 0,
                is_streak_win: true,
            }
        );
    }
}
