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
static VERSION: &str = "0.13";


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
    //pub totalhealpoints: f32,
    //pub lasthealpoints: f32,
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
    showbanner();
    //this is a routine for sharing struct data between all .rs files.
    populateconfigstruct();

    //print_target();
    let cfg = CONFIG.get().expect("Config is not initialized!");

    println!("blockentries: {}", cfg.blockentries);
    println!("targetfile: {}", cfg.target_file);

    // 1. Create a mutable instance OUTSIDE the loop
    let data = CollectedData::default();


    // 1. Skapa tailern en gång (startar i slutet av filen)
    let mut tailer = LineTailer::new(TARGET_FILE)?;


    // 2. Anropa i en loop eller vid event
    loop {
        if let Some(line) = tailer.read_next_new_line()? {
            //println!("{}", line);
            findpatterns(&line);
        }

        std::thread::sleep(std::time::Duration::from_millis(1));
    }
}


