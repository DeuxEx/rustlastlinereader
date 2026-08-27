#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use colored::Colorize;

use nix::libc::{EXIT_FAILURE, exit};
use serde::Deserialize;
use std::fs::{self, exists};

use std::process;

use std::path::PathBuf;
use std::sync::{OnceLock, Mutex};

use crate::CONFIG;

use crate::Config;


/*
#[derive(Debug, Deserialize)]
pub struct Config {
    // Store runtime paths as PathBuf
    pub target_file: PathBuf,
    pub fifo_pipe: PathBuf,
    pub blockentries: PathBuf,
    pub ammoburn: PathBuf,
    pub usecost: PathBuf,
}
*/




// 4. Accessing from any other function in your code:
pub fn print_target() {

    let config = CONFIG.get().unwrap().lock().unwrap();

    // Open the file dynamically using the runtime path
    if let Ok(content) = std::fs::read_to_string(&config.target_file) {
        println!("Successfully read {} bytes", content.len());

        println!("Target path: {}", config.target_file.display());
        println!("Block entries: {}", config.blockentries.display());
        println!("Ammoburn: {}", config.ammoburn.display());
        println!("Usecost: {}", config.usecost.display());
    }
}




pub fn populateconfigstruct() {

    //this is a routine for sharing struct data between all .rs files.

    // 1. Read config at startup (runtime)
    let loaded_path_from_file = String::from("config.ini");

    // 2. Populate the global struct
    let config = Config
    {
        target_file: PathBuf::from(loaded_path_from_file.clone()),
        fifo_pipe: PathBuf::from(loaded_path_from_file.clone()),
        ammoburn: PathBuf::from(loaded_path_from_file.clone()),
        blockentries: PathBuf::from(loaded_path_from_file.clone()),
        usecost: PathBuf::from(loaded_path_from_file.clone()),

    };
    CONFIG.set(Mutex::new(config)).ok();
}



impl Config {
    pub fn from_file(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(path)?;

        let mut target_file = String::new();
        let mut fifo_pipe = String::new();
        let mut blockentries = String::new();
        let mut ammoburn = String::new();
        let mut usecost = String::new();

        for line in content.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('=') || line.starts_with('[') {continue;}

            if let Some((key, value)) = line.split_once('=') {
                match key.trim() {
                    "target_file" => target_file = value.trim().to_string(),
                    "fifo_pipe" => fifo_pipe = value.trim().to_string(),
                    "blockentries" => blockentries = value.trim().to_string(),
                    "ammoburn" => ammoburn = value.trim().to_string(),
                    "usecost" => usecost = value.trim().to_string(),
                    _ => {}
                }
            }
        }

        Ok(Config {
            target_file: target_file.into(),
            fifo_pipe: fifo_pipe.into(),
            blockentries: blockentries.into(),
            ammoburn: ammoburn.into(),
            usecost: usecost.into(),
        })
    }
}
