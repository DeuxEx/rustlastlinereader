#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

mod patterns;
use patterns::{analyzestring, findpatterns};

mod filewatching;
use filewatching::{LineTailer};

mod inifilereader;
use inifilereader::{load_config_file};

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


static VERSION: &str = "0.17";


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
    pub target_file: String,
    pub fifo_pipe: String,
    pub blockentries: String,
    pub ammoburn: i32,
    pub usecost: f32,
    pub avatarname: String,
}



//ansi colors and details on text
use colored::*;



fn showbanner()
{
        println!("╔═══════════════════════════════════════════════════════════════════════════════════════╗");
        println!("║ Duxes File matching explorer for EU                  v. {}                    [2026]║", VERSION);
        println!("╚═══════════════════════════════════════════════════════════════════════════════════════╝");
}



//fn main() -> std::io::Result<()> {
fn main() -> Result<(), Box<dyn std::error::Error>> {

    showbanner();

    // 2. Anropa funktionen
    inifilereader::load_config_file()?;

    // Nu är CONFIG satt, du kan hämta data så här:
    let config = crate::CONFIG.get().unwrap().lock().unwrap();
    println!("{:?}", config.target_file);


    // Create a mutable instance OUTSIDE the loop
    // 1. FIRST: Initialize and set the global variable
    let data = CollectedData::default();
    COLLECTEDDATA.set(Mutex::new(data)).expect("Failed to initialize COLLECTEDDATA");



    // Skapa tailern en gång (startar i slutet av filen)
    //let mut tailer = LineTailer::new(TARGET_FILE)?;
    let mut tailer = LineTailer::new(config.target_file.clone())?;

    //println!("{}", config.target_file.clone());
    assert!(fs::exists(config.target_file.clone()).expect("Cant check existence of file"));


    // Loopa igenom radläsning från slutet med 1ms fördröjning
    loop {
            if let Some(line) = tailer.read_next_new_line()?
            {
                findpatterns(&line);
            }

            std::thread::sleep(std::time::Duration::from_millis(1));
        }
}


