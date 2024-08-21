use clap::{command, Parser};
use clap_stdin::FileOrStdin;
use ratatui::style::Color;
use regex::Regex;
use terminart::app::{App, AppResult};
use terminart::components::save_load::{AnsiData, SaveData};
use terminart::handler::{handle_key_events, handle_mouse_events};
use terminart::handler::{Event, EventHandler};
use terminart::tui::Tui;

use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;

use std::fs::File;
use std::io;
use std::path::Path;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// ANSI/text/.tart file to edit, can be read from stdin
    input: Option<FileOrStdin>,

    #[arg(short, long)]
    #[arg(value_parser = color_parser)]
    /// Colors to use in palette
    ///
    /// Formats: "rgb(1,2,3)" / "r,g,b" / #ffffff / #fff
    color: Option<Vec<Color>>,
}

fn main() -> AppResult<()> {
    // Create the application.
    let mut app = App::new();

    // Setup command line interface
    let cli = Cli::parse();

    // Load canvas from user input
    if let Some(input) = cli.input {
        if input.is_file() {
            let file_str = input.filename();

            let Ok(file) = File::open(file_str) else {
                println!("File does not exist: {:?}", file_str);
                app.quit();
                return Ok(());
            };

            if file_str.ends_with(".tart") {
                let data: SaveData = ciborium::from_reader(file)?;

                app.brush = data.brush;
                app.palette = data.palette;
                app.canvas.id_list = data.layers.iter().map(|l| l.id).collect();
                app.canvas.layers = data.layers;
                app.input_capture.last_file_name =
                    file_str.strip_suffix(".tart").map(|s| s.to_string());
            } else {
                app.canvas.layers[0].data = AnsiData::open_file(file);
                app.canvas.layers[0].name = "Imported Layer".into();
                app.input_capture.last_file_name = Path::new(file_str)
                    .file_stem()
                    .map(|s| s.to_string_lossy().into());
            }
        } else {
            let Ok(ansi) = input.contents() else {
                println!("Input not readable.");
                app.quit();
                return Ok(());
            };
            app.canvas.layers[0].data = AnsiData::read_str(ansi);
            app.canvas.layers[0].name = "Imported Layer".into();
        }
    }

    // Importing user colors
    if let Some(color_vec) = cli.color {
        app.palette
            .colors
            .iter_mut()
            .take(color_vec.len())
            .zip(color_vec)
            .for_each(|(og_color, user_color)| *og_color = user_color);
    }

    // Initialize the terminal user interface.
    let backend = CrosstermBackend::new(io::stderr());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(250 /* ms */);
    let mut tui = Tui::new(terminal, events)?;

    while app.running {
        // Render the interface.
        tui.render(&mut app)?;
        // Handle events.
        match tui.events.next()? {
            Event::Tick => {}
            Event::Key(key_event) => handle_key_events(key_event, &mut app)?,
            Event::Mouse(mouse_event) => handle_mouse_events(mouse_event, &mut app)?,
            Event::Resize(width, height) => app.resize(width, height),
            Event::Paste(s) => {
                // Take the first char from the clipboard and use it as the brush
                if let Some(c) = s.chars().next() {
                    app.brush.char = c;
                }
            }
        }
    }

    // Exit the interface
    tui.exit()?;
    Ok(())
}

fn color_parser(c_str: &str) -> core::result::Result<Color, String> {
    let full_hex_regex = Regex::new(r#"#([0-9a-fA-F].)(..)(..)"#).unwrap();
    let half_hex_regex = Regex::new(r#"#([0-9a-fA-F])(.)(.)"#).unwrap();
    let full_rgb_regex = Regex::new(r#"rgb\((\d+), ?(\d+), ?(\d+)\)"#).unwrap();
    let rgb_regex = Regex::new(r#"(\d+), ?(\d+), ?(\d+)"#).unwrap();

    for (i, regex) in [full_hex_regex, half_hex_regex, full_rgb_regex, rgb_regex]
        .into_iter()
        .enumerate()
    {
        let captures = regex.captures(c_str).map(|captures| {
            captures
                .iter() // All the captured groups
                .skip(1) // Skipping the complete match
                .flatten() // Ignoring all empty optional matches
                .map(|c| c.as_str()) // Grab the original strings
                .collect::<Vec<_>>() // Create a vector
        });

        if let Some(capture_vec) = captures {
            println!("{:?}", capture_vec);

            let result_vec: Vec<_> = match i {
                0 => capture_vec
                    .into_iter()
                    .map(|i| u8::from_str_radix(i, 16))
                    .collect(),
                1 => capture_vec
                    .into_iter()
                    .map(|i| u8::from_str_radix(&format!("{}{}", i, i), 16))
                    .collect(),
                2 | 3 => capture_vec.into_iter().map(|i| i.parse::<u8>()).collect(),
                _ => vec![],
            };

            if result_vec.iter().any(Result::is_err) {
                return Err(format!("Invalid color format: {}", c_str));
            } else {
                let vals: Vec<_> = result_vec.into_iter().map(Result::unwrap).collect();
                return Ok(Color::Rgb(vals[0], vals[1], vals[2]));
            }
        }
    }

    Err(format!("Invalid color format: {}", c_str))
}
