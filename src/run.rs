use std::{fs::File, path::Path};

use chrono::Local;
use inquire::Select;
use tracing::instrument;

use anyhow::Result;
use chrono::DateTime;
use prettytable::{
    color,
    format::{self, consts::FORMAT_BOX_CHARS, Alignment},
    row, Attr, Cell, Row, Table,
};
use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;

use crate::{
    error::Error,
    map::GunfightMap,
    menus::{DidWinOption, DisplayStatsOption, MainMenuOption},
    stats::{Stats, StatsGroup},
    Cli, CodVersion, Commands, DAY_FMT,
};

#[instrument(skip(cli))]
pub fn run(cli: Cli) -> Result<(), Error> {
    let file_path = cli.args.stats_path;
    tracing::info!(stats_path=?file_path, "checking if file exists");

    if !file_path.exists() {
        tracing::error!(stats_path=?file_path, "file does not exist");
        return Err(Error::StatsFileNotFound(file_path));
    }

    let mut games = load(&file_path);

    let result = match cli.command {
        Commands::Prompt => run_main_menu(&file_path, &mut games, &cli.args.cod_version),
        Commands::Graph => {
            // graph::draw_graph(games).unwrap();
            todo!();
        }
    };

    if let Err(error) = &result {
        tracing::error!("Encountered error, saving and exiting [{error}]");
    };

    save(&mut games, &file_path);

    result
}

#[instrument(skip(games))]
fn save(games: &mut Vec<GamePlayed>, file_path: &Path) {
    tracing::debug!("sorting games");
    games.sort_by(|a, b| a.date_time.cmp(&b.date_time));
    tracing::trace!(stats_path=?file_path, "writing to file");
    serde_json::to_writer_pretty(
        File::create(file_path).expect("stats file should be able to be created"),
        &games,
    )
    .expect("stats type should be able to be serialized into a writer");
    tracing::trace!(stats_path=?file_path, "wrote to file");
}

#[instrument]
fn load(file_path: &Path) -> Vec<GamePlayed> {
    tracing::trace!(stats_path=?file_path, "loading file data");
    let mut games: Vec<GamePlayed> = serde_json::from_str(
        &std::fs::read_to_string(file_path).expect("stats file should be able to be read"),
    )
    .expect("stats file should be able to be deserialized into type");
    games.sort_by(|a, b| a.date_time.cmp(&b.date_time));
    tracing::trace!(stats_path=?file_path, "file data loaded");
    games
}

#[instrument(skip(games))]
fn run_main_menu(
    file_path: &Path,
    games: &mut Vec<GamePlayed>,
    cod_version: &CodVersion,
) -> Result<(), Error> {
    let mut stats = Stats::new(games, Local::now(), cod_version)?;
    loop {
        match Select::new(
            &format!(
                "What would you like to do? (stat_sheet: {})",
                file_path.to_string_lossy()
            ),
            MainMenuOption::iter().collect(),
        )
        .prompt()?
        {
            MainMenuOption::DisplayStats => {
                option_display_stats(&stats, cod_version)?;
            }
            MainMenuOption::EnterGames => {
                option_enter_games(games, &mut stats, file_path, cod_version)?;
            }
            MainMenuOption::Back => break,
        }
    }
    Ok(())
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq)]
pub struct GamePlayed {
    pub map: GunfightMap,
    pub did_win: bool,
    pub date_time: DateTime<Local>,
}

impl PartialEq for GamePlayed {
    fn eq(&self, other: &Self) -> bool {
        self.map == other.map && self.did_win == other.did_win && self.date_time == other.date_time
    }
}

#[instrument(skip(stats))]
fn option_display_stats(stats: &Stats, cod_version: &CodVersion) -> Result<(), Error> {
    loop {
        match Select::new(
            "What would you like to do?",
            DisplayStatsOption::iter().collect(),
        )
        .prompt()?
        {
            DisplayStatsOption::Today => {
                let mut table = build_stat_table(&stats.today);
                table.set_format(*FORMAT_BOX_CHARS);
                table.printstd();
            }
            DisplayStatsOption::Lifetime => display_stats(stats),
            DisplayStatsOption::OneMap => {
                let map = GunfightMap::get_map_choice(cod_version)?;
                if let Some(map_stats) = stats.lifet.get_map_stats(&map) {
                    println!("{map}: {map_stats}");
                }
            }
            DisplayStatsOption::Maps => {
                stats.display_map_stats();
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
    Ok(())
}

#[instrument(skip(games, stats))]
fn option_enter_games(
    games: &mut Vec<GamePlayed>,
    stats: &mut Stats,
    file_path: &Path,
    cod_version: &CodVersion,
) -> Result<(), Error> {
    loop {
        match GunfightMap::get_map_choice(cod_version)? {
            GunfightMap::Back => break,
            map => {
                if let Some(map_stats) = stats.lifet.get_map_stats(&map) {
                    println!();
                    println!("{}: {} - {}", &map, &map_stats.wins, &map_stats.losses);
                    println!();
                }

                let time = Local::now();
                let did_win =
                    Select::new("Did you win?", DidWinOption::iter().collect()).prompt()?;
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
                    DidWinOption::Back => break,
                };

                games.push(game.clone());
                save(games, file_path);

                tracing::info!(game.map=%game.map, game.did_win=game.did_win, "recorded game");

                if game.did_win {
                    stats.add_win(&game, &game.date_time.format(DAY_FMT).to_string())?;
                } else {
                    stats.add_loss(&game, &game.date_time.format(DAY_FMT).to_string())?;
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
    Ok(())
}

#[instrument(skip(stats))]
fn display_stats(stats: &Stats) {
    println!();
    build_final_table(stats).printstd();
    println!();
}

#[instrument(skip(stats))]
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

    let lifetime = build_stat_table(&stats.lifet);
    let daily = build_stat_table(&stats.today);

    wrapper_table.set_format(*format::consts::FORMAT_CLEAN);
    wrapper_table.add_row(row![daily, lifetime]);

    wrapper_table
}

fn build_stat_table(stats: &StatsGroup) -> Table {
    let mut table = Table::new();
    table.add_row(Row::new(vec![
        Cell::new("Dub's"),
        Cell::new(&stats.wins.to_string())
            .with_style(Attr::Bold)
            .with_style(Attr::ForegroundColor(color::GREEN)),
    ]));
    table.add_row(Row::new(vec![
        Cell::new("Dub %"),
        Cell::new(&format!("{:.2}", &stats.get_win_percentage()))
            .with_style(Attr::Bold)
            .with_style(Attr::ForegroundColor(color::GREEN)),
    ]));
    table.add_row(Row::new(vec![
        Cell::new("Longest Dub Streak"),
        Cell::new(&stats.high_win_streak.to_string())
            .with_style(Attr::Bold)
            .with_style(Attr::ForegroundColor(color::GREEN)),
    ]));
    table.add_row(Row::new(vec![
        Cell::new("L's"),
        Cell::new(&stats.losses.to_string())
            .with_style(Attr::Bold)
            .with_style(Attr::ForegroundColor(color::RED)),
    ]));
    table.add_row(Row::new(vec![
        Cell::new("Longest L-L-L Streak"),
        Cell::new(&stats.high_loss_streak.to_string())
            .with_style(Attr::Bold)
            .with_style(Attr::ForegroundColor(color::RED)),
    ]));

    // table.set_format(*format::consts::FORMAT_NO_LINESEP_WITH_TITLE);
    table.set_format(*format::consts::FORMAT_NO_LINESEP_WITH_TITLE);
    table.set_format(*FORMAT_BOX_CHARS);

    table
}
