#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

mod patterns;
use patterns::{analyzestring, findpatterns};
mod watchdog;
use watchdog::read_last_line;
mod pipe;
use pipe::{ensure_fifo_exists, send_to_pipe};

use serde::Deserialize;
use std::fs::{self, exists};

use nix::{libc::file_handle, sys::stat::Mode};
use nix::unistd::mkfifo;

use notify_debouncer_full::{
    new_debouncer,
    notify::{EventKind, RecursiveMode, event::ModifyKind},
};

use std::io::{BufRead, BufReader, Seek, SeekFrom, Write};
use std::path::Path;
use std::sync::mpsc::channel;
use std::time::Duration;

use std::{
    array,
    fs::{File, OpenOptions},
};

//this is a routine for sharing struct data between all .rs files.
use std::sync::OnceLock;
pub static CONFIG: OnceLock<Config> = OnceLock::new();

#[derive(Debug, Deserialize)]
pub struct Config {
    pub target_file: String,
    pub fifo_pipe: String,
    // dina övriga variabler...
}


impl Config {
    pub fn from_file(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(path)?;

        let mut target_file = String::new();
        let mut fifo_pipe = String::new();

        for line in content.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('=') || line.starts_with('[') {
                continue;
            }

            if let Some((key, value)) = line.split_once('=') {
                match key.trim() {
                    "target_file" => target_file = value.trim().to_string(),
                    "fifo_pipe" => fifo_pipe = value.trim().to_string(),
                    _ => {}
                }
            }
        }

        Ok(Config {
            target_file,
            fifo_pipe,
        })
    }
}

static TARGET_FILE: &str = "/home/void/.local/share/Steam/steamapps/compatdata/3642750/pfx/drive_c/users/steamuser/Documents/Entropia Universe/chat.log";
static FIFO_PIPE: &str = "/tmp/min_pipe"; // Variabeln för sökvägen till pipen

fn populateconfigstruct() {
    //this is a routine for sharing struct data between all .rs files.
    // 1. Läs in inifilen
    let configfile = "config.ini";

    if exists(configfile).expect("REASON")
    {
        let conf = Config::from_file("config.ini").unwrap();
        // 2. Spara i CONFIG
        CONFIG.set(conf).unwrap();
        // 3. ANROPA funktionen från den andra filen!
        patterns::kor_analys();
    }
    else
    {
        println!("File doesnt exists {}", configfile);
    }

}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut newstring: String;

    //this is a routine for sharing struct data between all .rs files.
    populateconfigstruct();

    // 1. Make sure the PIPE exists (auto creating if its missing)
    ensure_fifo_exists(FIFO_PIPE)?;

    // 2. Make sure the target file exists
    let path = Path::new(TARGET_FILE);
    if !path.exists() {
        File::create(path)?;
    }

    let (tx, rx) = channel();
    let mut debouncer = new_debouncer(Duration::from_millis(200), None, tx)?;
    debouncer.watch(path, RecursiveMode::NonRecursive)?;

    println!(
        "Watchdog started for: {} (Pipe target: {})",
        path.display(),
        FIFO_PIPE
    );

    let mut last_line: String;

    for result in rx {
        match result {
            Ok(events) => {
                // Filtrera så att vi ENBART reagerar när faktiskt innehåll/data ändras
                let is_modified = events
                    .iter()
                    .any(|event| matches!(event.kind, EventKind::Modify(_)));

                if is_modified {
                    match read_last_line(path) {
                        Ok(Some(line)) => {
                            last_line = line;
                            //analyzestring(&last_line, fifo_pipe);

                            //let newstring = formatstring(&last_line);
                            //analyzestring(&newstring);
                            analyzestring(&last_line);
                        }
                        Ok(None) => println!("The file is empty."),
                        Err(e) => eprintln!("Error reading: {}", e),
                    }
                }
            }
            Err(errors) => {
                for err in errors {
                    eprintln!("Debounce error: {:?}", err);
                }
            }
        }
    }

    Ok(())
}
