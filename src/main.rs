use chrono::{DateTime, Local};
use inquire::Select;
use menus::{DidWinOption, DisplayStatsOption, MainMenuOption};
use prettytable::{
    color,
    format::{self, consts::FORMAT_BOX_CHARS, Alignment},
    row, Attr, Cell, Row, Table,
};
use serde::{Deserialize, Serialize};
use stats::Stats;
use std::{
    env,
    fs::{self, File},
};
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter};

const DAY_FMT: &str = "%m-%d-%Y";

mod menus;
mod stats;

fn main() {
    let file_path = env::var("STATS_SHEET").unwrap_or("stat_sheet_test.json".to_string());

    let mut games = load(&file_path);
    let mut stats = Stats::new(&mut games, Local::now());

    loop {
        match Select::new(
            &format!("What would you like to do? (stat_sheet: {})", &file_path),
            MainMenuOption::iter().collect(),
        )
        .prompt()
        .unwrap()
        {
            MainMenuOption::DisplayStats => {
                option_display_stats(&stats);
            }
            MainMenuOption::EnterGames => option_enter_games(&mut games, &mut stats, &file_path),
            MainMenuOption::Back => break,
        }
    }

    save(&mut games, &file_path);
}

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

#[derive(Serialize, Deserialize, Debug, Clone, Eq)]
pub struct GamePlayed {
    map: GunfightMap,
    did_win: bool,
    date_time: DateTime<Local>,
}

impl PartialEq for GamePlayed {
    fn eq(&self, other: &Self) -> bool {
        self.map == other.map && self.did_win == other.did_win && self.date_time == other.date_time
    }
}

fn option_display_stats(stats: &Stats) {
    loop {
        match Select::new(
            "What would you like to do?",
            DisplayStatsOption::iter().collect(),
        )
        .prompt()
        .unwrap()
        {
            DisplayStatsOption::Today => {
                let mut table = build_stat_table(
                    stats.today.wins,
                    stats.today.losses,
                    stats.today.high_win_streak,
                    stats.today.high_loss_streak,
                );
                table.set_format(*FORMAT_BOX_CHARS);
                table.printstd();
            }
            DisplayStatsOption::Lifetime => display_stats(&stats),
            DisplayStatsOption::OneMap => {
                let map = &Select::new("Which Map?", GunfightMap::iter().filter(|m| m != &GunfightMap::Back).collect())
                    .prompt()
                    .unwrap();
                if let Some(map_stats) = stats.lifet.get_map_stats(map) {
                    println!();
                    println!("{}: {}", map, map_stats);
                    println!();
                }
                
            },
            DisplayStatsOption::Maps => {
                println!();
                println!("Lifetime:\n---");
                stats.lifet.get_all_map_stats().iter().for_each(|item| {
                    println!("{}: {}", item.0, item.1);
                });
                println!();
                println!("Today:\n---");
                stats.today.get_all_map_stats().iter().for_each(|item| {
                    println!("{}: {}", item.0, item.1);
                });
                println!();
            }
            DisplayStatsOption::CurrentStreak => {
                println!(
                    "You are on a {} streak of {}.",
                    if stats.lifet.last_was_win {
                        "Winning"
                    } else {
                        "Losing"
                    },
                    if stats.lifet.last_was_win {
                        stats.lifet.win_streak
                    } else {
                        stats.lifet.loss_streak
                    },
                );
            }
            DisplayStatsOption::Back => break,
        }
    }
}

fn option_enter_games(games: &mut Vec<GamePlayed>, stats: &mut Stats, file_path: &str) {
    loop {
        match Select::new("Which Map?", GunfightMap::iter().collect())
            .prompt()
            .unwrap()
        {
            GunfightMap::Back => break,
            map => {
                let map_stats = stats.lifet.get_map_stats(&map).expect(
                    "Could not find stats for this map for some reason. This is probably a bug.",
                );
                println!();
                println!("{}: {} - {}", &map, &map_stats.wins, &map_stats.losses);
                println!();

                let time = Local::now();
                let did_win = Select::new("Did you win?", DidWinOption::iter().collect())
                    .prompt()
                    .unwrap();

                let game: GamePlayed;

                match did_win {
                    DidWinOption::Yes => {
                        game = GamePlayed {
                            map,
                            did_win: true,
                            date_time: time,
                        };
                    }
                    DidWinOption::No => {
                        game = GamePlayed {
                            map,
                            did_win: false,
                            date_time: time,
                        };
                    }
                    DidWinOption::Back => break,
                }

                games.push(game.clone());
                save(games, file_path);
                if game.did_win {
                    stats.add_win(&game, &game.date_time.format(DAY_FMT).to_string());
                } else {
                    stats.add_loss(&game, &game.date_time.format(DAY_FMT).to_string());
                }

                display_stats(stats);
                println!(
                    "{} on {} saved. {} Streak now {}.",
                    if game.did_win { "Win" } else { "Loss" },
                    game.map,
                    if stats.today.last_was_win {
                        "Winning"
                    } else {
                        "Losing"
                    },
                    if stats.today.last_was_win {
                        stats.lifet.win_streak
                    } else {
                        stats.lifet.loss_streak
                    },
                );
                {
                    let map_stats = stats.lifet.get_map_stats(&game.map).expect(
                                "Could not find stats for this map for some reason. This is probably a bug.",
                            );
                    println!();
                    println!("{}: {} - {}", &game.map, &map_stats.wins, &map_stats.losses);
                    println!();
                }

            }
        }
    }
}

fn display_stats(stats: &Stats) {
    println!();
    build_final_table(stats).printstd();
    println!();
}

fn build_final_table(stats: &Stats) -> Table {
    let mut lifetime_title_cell = Cell::new("Lifetime Stats")
        .with_style(Attr::Bold)
        // .with_style(Attr::Italic(true))
        .with_style(Attr::ForegroundColor(color::CYAN));
    lifetime_title_cell.align(Alignment::CENTER);
    let mut today_title_cell = Cell::new("Today's Stats")
        .with_style(Attr::Bold)
        // .with_style(Attr::Italic(true))
        .with_style(Attr::ForegroundColor(color::CYAN));
    today_title_cell.align(Alignment::CENTER);

    let mut wrapper_table = Table::new();
    wrapper_table.add_row(Row::new(vec![today_title_cell, lifetime_title_cell]));

    let lifetime = build_stat_table(
        stats.lifet.wins,
        stats.lifet.losses,
        stats.lifet.high_win_streak,
        stats.lifet.high_loss_streak,
    );
    let daily = build_stat_table(
        stats.today.wins,
        stats.today.losses,
        stats.today.high_win_streak,
        stats.today.high_loss_streak,
    );

    wrapper_table.set_format(*format::consts::FORMAT_CLEAN);
    wrapper_table.add_row(row![daily, lifetime]);

    wrapper_table
}

fn build_stat_table(wins: usize, loses: usize, win_streak: usize, loss_streak: usize) -> Table {
    let mut table = Table::new();
    table.add_row(Row::new(vec![
        Cell::new("Dub's"),
        Cell::new(&wins.to_string())
            .with_style(Attr::Bold)
            .with_style(Attr::ForegroundColor(color::GREEN)),
    ]));
    table.add_row(Row::new(vec![
        Cell::new("Longest Dub Streak"),
        Cell::new(&win_streak.to_string())
            .with_style(Attr::Bold)
            .with_style(Attr::ForegroundColor(color::GREEN)),
    ]));
    table.add_row(Row::new(vec![
        Cell::new("L's"),
        Cell::new(&loses.to_string())
            .with_style(Attr::Bold)
            .with_style(Attr::ForegroundColor(color::RED)),
    ]));
    table.add_row(Row::new(vec![
        Cell::new("Longest L-L-L Streak"),
        Cell::new(&loss_streak.to_string())
            .with_style(Attr::Bold)
            .with_style(Attr::ForegroundColor(color::RED)),
    ]));

    // table.set_format(*format::consts::FORMAT_NO_LINESEP_WITH_TITLE);
    table.set_format(*format::consts::FORMAT_NO_LINESEP_WITH_TITLE);
    table.set_format(*FORMAT_BOX_CHARS);

    table
}

fn save(games: &mut Vec<GamePlayed>, file_path: &str) {
    games.sort_by(|a, b| a.date_time.cmp(&b.date_time));
    serde_json::to_writer(File::create(file_path).unwrap(), &games).unwrap();
}

fn load(file_path: &str) -> Vec<GamePlayed> {
    let mut games: Vec<GamePlayed> =
        serde_json::from_str(&fs::read_to_string(file_path).unwrap()).unwrap();
    games.sort_by(|a, b| a.date_time.cmp(&b.date_time));
    games
}
