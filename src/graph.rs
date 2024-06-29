// use serde::Deserialize;

// #[derive(Deserialize)]
// struct DailyData {
//     #[serde(default)]
//     new_cases: f64,
//     #[serde(default)]
//     total_cases: f64,
// }
//
// #[derive(Deserialize)]
// struct CountryData {
//     data: Vec<DailyData>,
// }

// const OUT_FILE_NAME: &str = "plotters-doc-data/win_loss.svg";
// pub fn draw_graph(games: &[GamePlayed]) -> Result<()> {
//     let root = SVGBackend::new(OUT_FILE_NAME, (1024, 768)).into_drawing_area();
//     root.fill(&WHITE)?;
//
//     let (upper, lower) = root.split_vertically(750);
//
//     lower.titled(
//         "Win Loss",
//         ("sans-serif", 10).into_font().color(&BLACK.mix(0.5)),
//     )?;
//
//     let mut chart = ChartBuilder::on(&upper)
//         .caption("Win Loss", ("sans-serif", (5).percent_height()))
//         .set_label_area_size(LabelAreaPosition::Left, (8).percent())
//         .set_label_area_size(LabelAreaPosition::Bottom, (4).percent())
//         .margin((1).percent())
//         .build_cartesian_2d(
//             (20u32..5000_0000u32)
//                 .log_scale()
//                 .with_key_points(vec![50, 100, 1000, 10000, 100000, 1000000, 10000000]),
//             (0u32..50_0000u32)
//                 .log_scale()
//                 .with_key_points(vec![10, 50, 100, 1000, 10000, 100000, 200000]),
//         )?;
//
//     chart
//         .configure_mesh()
//         .x_desc("Time")
//         .y_desc("Win")
//         .draw()?;
//
//     let data: std::collections::HashMap<GunfightMap, bool> = games.iter().map(|g| (g.map, g.did_win)).collect::<HashMap<_, _>>();
//     for (idx, series) in GunfightMap::iter()
//         .enumerate()
//     {
//         let color = Palette99::pick(idx).mix(0.9);
//         chart
//             .draw_series(LineSeries::new(
//                 data[series],
//                 color.stroke_width(3),
//             ))?
//             .label(series)
//             .legend(move |(x, y)| Rectangle::new([(x, y - 5), (x + 10, y + 5)], color.filled()));
//     }
//
//     chart.configure_series_labels().border_style(BLACK).draw()?;
//
//     // To avoid the IO failure being ignored silently, we manually call the present function
//     root.present().expect("Unable to write result to file, please make sure 'plotters-doc-data' dir exists under current dir");
//     println!("Result has been saved to {}", OUT_FILE_NAME);
//     Ok(())
// }
