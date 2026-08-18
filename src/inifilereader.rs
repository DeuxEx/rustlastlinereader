#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use colored::Colorize;

use serde::Deserialize;
use std::fs::{self, exists};

use crate::CONFIG;



#[derive(Debug, Deserialize)]
pub struct Config {
    pub target_file: String,
    pub fifo_pipe: String,
    // dina övriga variabler...
}



pub fn populateconfigstruct() {
    //this is a routine for sharing struct data between all .rs files.


    // 1. Läs in inifilen
    let configfile = "config.ini";

    if exists(configfile).expect("REASON")
    {
        let conf = Config::from_file("config.ini").unwrap();
        // 2. Spara i CONFIG
        CONFIG.set(conf).unwrap();
        // 3. ANROPA funktionen från den andra filen!
        //patterns::kor_analys();


    }
    else
    {
        eprintln!("[File doesnt exists: {}]", configfile.red().bold());
    }
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
