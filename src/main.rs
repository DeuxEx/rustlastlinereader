#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

mod patterns;
use patterns::{analyzestring, findpatterns};

mod filewatching;
use filewatching::{LineTailer};

mod inifilereader;
use inifilereader::{populateconfigstruct, Config, print_target};

mod pipe;
use pipe::{ensure_fifo_exists, send_to_pipe};

use std::fs::{self, exists};

use nix::{libc::file_handle, sys::stat::Mode};
use nix::unistd::mkfifo;

use std::path::PathBuf;
use std::env;

use std::{
    array,
    fs::{File, OpenOptions},
};


static TARGET_FILE: &str = "/home/void/.local/share/Steam/steamapps/compatdata/3642750/pfx/drive_c/users/steamuser/Documents/Entropia Universe/chat.log";
static VERSION: &str = "0.14";


//this is a routine for sharing struct data between all .rs files.
use std::sync::OnceLock;
pub static CONFIG: OnceLock<Config> = OnceLock::new();

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

    //Get the config struct variables
    let cfg = CONFIG.get().expect("Config is not initialized!");

    println!("blockentries: {}", cfg.blockentries);
    println!("targetfile: {}", cfg.target_file);

    // Create a mutable instance OUTSIDE the loop
    // 1. FIRST: Initialize and set the global variable
    let data = CollectedData::default();


    COLLECTEDDATA
    .set(Mutex::new(data))
    .expect("Failed to initialize COLLECTEDDATA");



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


