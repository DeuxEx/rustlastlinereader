#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

mod patterns;
use patterns::{analyzestring, findpatterns};

mod filewatching;
use filewatching::{LineTailer};

mod pipe;
use pipe::{ensure_fifo_exists, send_to_pipe};

use serde::Deserialize;
use std::fs::{self, exists};

use nix::{libc::file_handle, sys::stat::Mode};
use nix::unistd::mkfifo;


use std::io::{BufRead, BufReader, Seek, SeekFrom, Write};
use std::path::Path;

use std::path::PathBuf;
use std::env;


use std::{
    array,
    fs::{File, OpenOptions},
};

static TARGET_FILE: &str = "/home/void/.local/share/Steam/steamapps/compatdata/3642750/pfx/drive_c/users/steamuser/Documents/Entropia Universe/chat.log";


//this is a routine for sharing struct data between all .rs files.
use std::sync::OnceLock;
pub static CONFIG: OnceLock<Config> = OnceLock::new();

static FIFO_PIPE: &str = "/tmp/min_pipe"; // Variabeln för sökvägen till pipen

//ansi colors and details on text
use colored::*;



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





fn startbanner()
{
    println!("┌────────────────────────────────────────┐");
    println!("│         DEUX FILE READER 0.1           │");
    println!("└────────────────────────────────────────┘");
}




fn populateconfigstruct() {
    //this is a routine for sharing struct data between all .rs files.

    startbanner();


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
        eprintln!("[File doesnt exists: {}]", configfile.red().bold());
    }
}





fn main() -> std::io::Result<()> {
    //this is a routine for sharing struct data between all .rs files.
    populateconfigstruct();


    // 1. Skapa tailern en gång (startar i slutet av filen)
    let mut tailer = LineTailer::new(TARGET_FILE)?;

    // 2. Anropa i en loop eller vid event
    loop {
        if let Some(line) = tailer.read_next_new_line()? {
            println!("Ny rad: {}", line);
            findpatterns(&line);
        }

        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}
