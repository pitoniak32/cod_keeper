use chrono::{DateTime, Local};
use inquire::{Confirm, Select};
use prettytable::{
    color,
    format::{self, consts::FORMAT_BOX_CHARS, Alignment},
    row, Attr, Cell, Row, Table,
};
use serde::{Deserialize, Serialize};
use stats::Stats;
use std::fs::{self, File};
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter};

const DAY_FMT: &str = "%m-%d-%Y";
const FILE_NAME: &str = "stat_sheet_real.json";

mod stats;

fn main() {
    let mut games = load();
    let mut stats = Stats::new(&mut games, Local::now());

    // TODO: pick which stat sheet to use.
    // TODO: add back to menu option.

    match Select::new(
        &format!("What would you like to do? (stat_sheet: {})", FILE_NAME),
        MainMenuOption::iter().collect(),
    )
    .prompt()
    .unwrap()
    {
        MainMenuOption::DisplayStats => {
            option_display_stats(&stats);
        }
        MainMenuOption::EnterGames => option_enter_games(&mut games, &mut stats),
    }

    save(&mut games);
}

#[derive(Serialize, Deserialize, Debug, EnumIter, Display, PartialEq, Eq)]
enum MainMenuOption {
    EnterGames,
    DisplayStats,
}

#[derive(Serialize, Deserialize, Debug, EnumIter, Display, PartialEq, Eq)]
enum DisplayStatsOption {
    Lifetime,
    CurrentStreak,
    Day,
}

#[derive(Serialize, Deserialize, Debug, Clone, EnumIter, Display, PartialEq, Eq)]
enum GunfightMap {
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
    None,
}

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
    match Select::new(
        "What would you like to do?",
        DisplayStatsOption::iter().collect(),
    )
    .prompt()
    .unwrap()
    {
        DisplayStatsOption::Day => {
            // let choice = Select::new("Which stats?", days).prompt().unwrap();

            // let todays_wins = day_stats.iter().filter(|g| g.did_win).count();
            // let mut table = Table::new();
            // table.add_row(Row::new(vec![Cell::new("wins"), Cell::new("loses")]));
            // table.add_row(Row::new(vec![
            //     Cell::new(&todays_wins.to_string()),
            //     Cell::new(&(day_stats.iter().count() - todays_wins).to_string()),
            // ]));

            // table.set_format(*FORMAT_BOX_CHARS);
            // table.printstd();
        }
        DisplayStatsOption::Lifetime => display_stats(&stats),
        DisplayStatsOption::CurrentStreak => todo!(),
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, EnumIter, Display, PartialEq, Eq)]
enum DidWinOption {
    Yes,
    No,
    Quit,
}

fn option_enter_games(games: &mut Vec<GamePlayed>, stats: &mut Stats) {
    let mut did_win: DidWinOption = Select::new("Did you win?", DidWinOption::iter().collect())
        .prompt()
        .unwrap();
    while did_win != DidWinOption::Quit {
        if did_win != DidWinOption::Quit {
            let map = Select::new("Which Map?", GunfightMap::iter().collect())
                .prompt()
                .unwrap();
            let time = Local::now();
            let game = match did_win {
                DidWinOption::Yes => GamePlayed {
                    map,
                    did_win: true,
                    date_time: time,
                },
                DidWinOption::No => GamePlayed {
                    map,
                    did_win: false,
                    date_time: time,
                },
                DidWinOption::Quit => break,
            };
            games.push(game.clone());
            save(games);
            if game.did_win {
                stats.add_win(&game, &time.format(DAY_FMT).to_string());
            } else {
                stats.add_loss(&game, &time.format(DAY_FMT).to_string());
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
            println!();
            did_win = Select::new("Did you win?", DidWinOption::iter().collect())
                .prompt()
                .unwrap();
        }
    }

    display_stats(stats);
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

fn save(games: &mut Vec<GamePlayed>) {
    games.sort_by(|a, b| a.date_time.cmp(&b.date_time));
    serde_json::to_writer(File::create(FILE_NAME).unwrap(), &games).unwrap();
}

fn load() -> Vec<GamePlayed> {
    let mut games: Vec<GamePlayed> =
        serde_json::from_str(&fs::read_to_string(FILE_NAME).unwrap()).unwrap();
    games.sort_by(|a, b| a.date_time.cmp(&b.date_time));
    games
}
