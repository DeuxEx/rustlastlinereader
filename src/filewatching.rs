#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]


use crate::analyzestring;


use std::{array, fs::{File, OpenOptions}};
use std::io::{BufRead, BufReader, Seek, SeekFrom, Write};
use std::path::Path;

use std::sync::mpsc::channel;
use std::time::Duration;


use notify_debouncer_full::{
    new_debouncer,
    notify::{EventKind, RecursiveMode, event::ModifyKind},
};


static TARGET_FILE: &str = "/home/void/.local/share/Steam/steamapps/compatdata/3642750/pfx/drive_c/users/steamuser/Documents/Entropia Universe/chat.log";



/// Läser den sista icke-tomma raden ur en fil genom att söka sig bakåt från slutet
pub fn read_last_line<P: AsRef<Path>>(path: P) -> std::io::Result<Option<String>> {
    let mut file = File::open(path)?;
    let file_size = file.metadata()?.len();

    if file_size == 0 {
        return Ok(None);
    }

    let read_size = file_size.min(1024) as i64;
    file.seek(SeekFrom::End(-read_size))?;

    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().collect::<Result<_, _>>()?;

    Ok(lines.into_iter().rev().find(|line| !line.trim().is_empty()))
}






pub fn startwatching() -> Result<(), Box<dyn std::error::Error>>
{
    let mut newstring: String;

    // 2. Make sure the target file exists
    let path = Path::new(TARGET_FILE);
    if !path.exists() {
        File::create(path)?;
    }

    let (tx, rx) = channel();
    let mut debouncer = new_debouncer(Duration::from_millis(200), None, tx)?;
    debouncer.watch(path, RecursiveMode::NonRecursive)?;


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


