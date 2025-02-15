use std::{
    error::Error,
    io::{BufRead, BufReader},
    path::PathBuf,
    sync::{Arc, RwLock},
};

use crate::state::AppState;
use clap::{command, Parser};
use ratatui::DefaultTerminal;

mod events;
mod state;
mod ui;

enum InputSource {
    File(PathBuf),
    Stdin,
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    input: Option<String>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    let terminal = ratatui::init();
    run(terminal, cli)?;
    ratatui::restore();
    Ok(())
}

fn run(mut terminal: DefaultTerminal, cli: Cli) -> Result<(), Box<dyn Error>> {
    let input = cli
        .input
        .map(|s| InputSource::File(PathBuf::from(s)))
        .unwrap_or(InputSource::Stdin);

    let state = Arc::new(RwLock::new(AppState::new()));

    let (tx, rx) = std::sync::mpsc::channel();
    std::thread::spawn(move || read_from_input(input, tx));

    loop {
        if state.read().unwrap().quit {
            break Ok(());
        }

        while let Ok(line) = rx.try_recv() {
            state.write().unwrap().add_line(line);
        }

        terminal.draw(|f| {
            let _ = ui::render(f, &state.clone());
        })?;

        if let Some(action) = events::poll_events(state.clone())? {
            let mut state_mut = state.write().unwrap();
            let _ = &mut state_mut.handle_action(action);
        }
    }
}

fn read_from_input(input: InputSource, tx: std::sync::mpsc::Sender<String>) {
    match input {
        InputSource::File(path) => {
            let file = std::fs::File::open(path).unwrap();
            let reader = BufReader::new(file);
            for line in reader.lines() {
                let _ = tx.send(line.unwrap());
            }
        }
        InputSource::Stdin => {
            let reader = BufReader::new(std::io::stdin());
            for line in reader.lines() {
                let _ = tx.send(line.unwrap());
            }
        }
    }
}
