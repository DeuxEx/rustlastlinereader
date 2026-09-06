#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use colored::Colorize;

use nix::libc::{EXIT_FAILURE, exit};

use serde::Deserialize;

use std::fs::{self, exists};

use std::process;

use std::sync::{OnceLock, Mutex};

use crate::CONFIG;
use crate::Config;


use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;




pub fn load_config_file() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open("config.ini")?;
    let reader = BufReader::new(file);

    let mut target_file = String::new();
    let mut fifo_pipe = String::new();
    let mut blockentries = String::new();
    let mut avatarname = String::new();
    let mut ammoburn = 0;
    let mut usecost = 0.0;

    for line in reader.lines() {
        let line = line?;
        let trimmed = line.trim();

        if trimmed.is_empty() || trimmed.starts_with('#') || trimmed.starts_with(';') {
            continue;
        }

        if let Some((key, value)) = trimmed.split_once('=') {
            let key = key.trim();

            // Thoroughly cleans quotes, Windows carriage returns (\r), and residual spaces
            let value = value.trim().trim_matches('"').trim_end_matches('\r').trim();

            // SKRIV UT FÖR ATT FELSÖKA:
            println!("Läst nyckel: [{}] -> Värde: [{:?}]", key, value);

            match key {
                "target_file" => target_file = value.to_string(),
                "fifo_pipe" => fifo_pipe = value.to_string(),
                "blockentries" => blockentries = value.to_string(),
                "ammoburn" => ammoburn = value.parse().unwrap_or(0),
                "usecost" => usecost = value.parse().unwrap_or(0.0),
                "avatarname" => avatarname = value.to_string(),
                _ => {}
            }
        }
    }

    let config = Config {
        target_file,
        fifo_pipe,
        blockentries,
        ammoburn,
        usecost,
        avatarname,
    };

    let _ = CONFIG.set(config);
    Ok(())
}
