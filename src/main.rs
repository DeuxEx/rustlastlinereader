#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]


mod patterns;
use patterns::findpatterns;
use patterns::analyzestring;
mod watchdog;
use watchdog::read_last_line;
mod pipe;
use pipe::ensure_fifo_exists;
use pipe::send_to_pipe;


use nix::sys::stat::Mode;
use nix::unistd::mkfifo;
use notify_debouncer_full::{new_debouncer,notify::{event::ModifyKind, EventKind, RecursiveMode},};
use std::{array, fs::{File, OpenOptions}};
use std::io::{BufRead, BufReader, Seek, SeekFrom, Write};
use std::path::Path;
use std::sync::mpsc::channel;
use std::time::Duration;



static TARGET_FILE: &str = "/home/void/.local/share/Steam/steamapps/compatdata/3642750/pfx/drive_c/users/steamuser/Documents/Entropia Universe/chat.log";
static FIFO_PIPE: &str = "/tmp/min_pipe"; // Variabeln för sökvägen till pipen





fn main() -> Result<(), Box<dyn std::error::Error>> {

    let mut newstring: String;

    // 1. Säkerställ att pipen finns (skapas automatiskt om den saknas)
    ensure_fifo_exists(FIFO_PIPE)?;

    // 2. Säkerställ att målfilen finns
    let path = Path::new(TARGET_FILE);
    if !path.exists() {
        File::create(path)?;
    }

    let (tx, rx) = channel();
    let mut debouncer = new_debouncer(Duration::from_millis(200), None, tx)?;
    debouncer.watch(path, RecursiveMode::NonRecursive)?;

    println!("Watchdog started for: {} (Pipe target: {})",path.display(),FIFO_PIPE);

    let mut last_line: String;


    for result in rx {
        match result {
            Ok(events) => {
                // Filtrera så att vi ENBART reagerar när faktiskt innehåll/data ändras
                let is_modified = events.iter().any(|event| { matches!(event.kind, EventKind::Modify(_)) });

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

