#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use colored::Colorize;

use nix::libc::{EXIT_FAILURE, exit};
use serde::Deserialize;
use std::fs::{self, exists};

use std::process;
use std::sync::OnceLock;

use crate::CONFIG;




#[derive(Debug, Deserialize)]
pub struct Config {
    pub target_file: String,
    pub fifo_pipe: String,
    pub blockentries: String,
    // dina övriga variabler...
}





// 4. Accessing from any other function in your code:
pub fn print_target() {
    let cfg = CONFIG.get().expect("Config is not initialized!");
    println!("Target path: {}", cfg.target_file);
    println!("Block entries: {}", cfg.blockentries);
}




pub fn populateconfigstruct() {

    let path = std::env::current_dir();
    println!("{}", path.expect("REASON").display());

    //this is a routine for sharing struct data between all .rs files.
    // 1. Läs in inifilen
    let configfile = "config.ini";
    //let configfile = "{}/config.ini",path.expect("REASON").display();



    if exists(configfile).expect("REASON")
    {
        let conf = Config::from_file("config.ini").unwrap();
        // 2. Spara i CONFIG
        CONFIG.set(conf).unwrap();
        // 3. ANROPA funktionen från den andra filen!
        //patterns::kor_analys();

        //In Rust, accessing a String field directly without & transfers ownership out of the struct.
        //Once a field is moved, the struct becomes partially invalid and you cannot borrow from it anymore.
    }
    else
    {
        eprintln!("[File doesnt exists: {}]", configfile.red().bold());
        process::exit(0x0100);
    }
}



impl Config {
    pub fn from_file(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(path)?;

        let mut target_file = String::new();
        let mut fifo_pipe = String::new();
        let mut blockentries = String::new();

        for line in content.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('=') || line.starts_with('[') {
                continue;
            }

            if let Some((key, value)) = line.split_once('=') {
                match key.trim() {
                    "target_file" => target_file = value.trim().to_string(),
                    "fifo_pipe" => fifo_pipe = value.trim().to_string(),
                    "blockentries" => blockentries = value.trim().to_string(),
                    _ => {}
                }
            }
        }

        Ok(Config {
            target_file,
            fifo_pipe,
            blockentries,
        })
    }
}
