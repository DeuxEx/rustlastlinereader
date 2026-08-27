#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

mod patterns;
use patterns::{analyzestring, findpatterns};

mod filewatching;
use filewatching::{LineTailer};

mod inifilereader;
//use inifilereader::{populateconfigstruct, Config, print_target};
use inifilereader::{populateconfigstruct, print_target};

mod pipe;
use pipe::{ensure_fifo_exists, send_to_pipe};

use std::fs::{self, exists};

use nix::{libc::file_handle, sys::stat::Mode};
use nix::unistd::mkfifo;

use std::path::PathBuf;
use std::env;

use serde::Deserialize;

use std::{
    array,
    fs::{File, OpenOptions},
};


static TARGET_FILE: &str = "/home/void/.local/share/Steam/steamapps/compatdata/3642750/pfx/drive_c/users/steamuser/Documents/Entropia Universe/chat.log";
static VERSION: &str = "0.16";


//this is a routine for sharing struct data between all .rs files.
use std::sync::OnceLock;
pub static CONFIG: OnceLock<Mutex<Config>> = OnceLock::new();

use std::sync::Mutex;
pub static COLLECTEDDATA: OnceLock<Mutex<CollectedData>> = OnceLock::new();


#[derive(Debug, Default)]
pub struct CollectedData {
    pub totaldamage: f32,
    pub lastdamage: f32,
    pub totallootvalue: f32,
    pub lastlootvalue: f32,
    pub totalshots: i32,
    pub lastmobshots: i32,
    pub numberofkills: i32,
}


#[derive(Debug, Deserialize)]
pub struct Config {
    // Store runtime paths as PathBuf
    pub target_file: PathBuf,
    pub fifo_pipe: PathBuf,
    pub blockentries: PathBuf,
    pub ammoburn: PathBuf,
    pub usecost: PathBuf,
}



static FIFO_PIPE: &str = "/tmp/min_pipe"; // Variabeln för sökvägen till pipen

//ansi colors and details on text
use colored::*;





fn showbanner()
{
        println!("╔═══════════════════════════════════════════════════════════════════════════════════════╗");
        println!("║ Duxes File matching explorer for EU                  v. {}                    [2026]║", VERSION);
        println!("╚═══════════════════════════════════════════════════════════════════════════════════════╝");
}



fn main() -> std::io::Result<()> {

    //show startbanner
    showbanner();

    //this is a routine for sharing struct data between all .rs files.
    populateconfigstruct();


    let config = CONFIG.get().unwrap().lock().unwrap();

    // Open the file dynamically using the runtime path
    if let Ok(content) = std::fs::read_to_string(&config.target_file)
    {
        println!("Successfully read {} bytes", content.len());
    }


    // Create a mutable instance OUTSIDE the loop
    // 1. FIRST: Initialize and set the global variable
    let data = CollectedData::default();
    COLLECTEDDATA.set(Mutex::new(data)).expect("Failed to initialize COLLECTEDDATA");


    // Skapa tailern en gång (startar i slutet av filen)
    let mut tailer = LineTailer::new(TARGET_FILE)?;


    // Loopa igenom radläsning från slutet med 1ms fördröjning
    loop {
        if let Some(line) = tailer.read_next_new_line()?
        {
            findpatterns(&line);
        }

        std::thread::sleep(std::time::Duration::from_millis(1));
    }
}


