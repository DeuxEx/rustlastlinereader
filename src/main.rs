#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

mod patterns;
use patterns::{analyzestring, findpatterns};

mod filewatching;
use filewatching::{LineTailer};

mod inifilereader;
use inifilereader::{load_config_file};

mod debug;
use debug::{create_mock_log_file};

mod pipe;
use pipe::{ensure_fifo_exists, send_to_pipe};

use std::fs::{self, exists};

use nix::{libc::file_handle, sys::stat::Mode};
use nix::unistd::mkfifo;

use std::path::PathBuf;
use std::env;

use serde::Deserialize;

use std::{array, fs::{File, OpenOptions},
};


use std::time::Duration;
static VERSION: &str = "0.19";


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

impl Config {
    // Hjälpmetod som automatiskt ger dig en Vec<String>
    pub fn blockentries_vec(&self) -> Vec<String> {
        self.blockentries
        .split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect()
    }
}


//ansi colors and details on text
use colored::*;



pub fn update_collected_data<F>(f: F)
where
F: FnOnce(&mut CollectedData),
{
    let data_cell = COLLECTEDDATA.get().expect("CollectedData not initialized");
    let mut data = data_cell.lock().unwrap();
    f(&mut data);
}


pub fn read_collected_data<F, T>(f: F) -> T
where
F: FnOnce(&CollectedData) -> T,
{
    let data_cell = COLLECTEDDATA.get().expect("CollectedData not initialized");
    let data = data_cell.lock().unwrap();
    f(&data) // Skickar med en referens till datan och returnerar det resultatet funktionen vill ha
}


pub fn read_any_data<T, F, R>(cell: &std::sync::OnceLock<std::sync::Mutex<T>>, f: F) -> R
where
F: FnOnce(&T) -> R,
{
    let data_cell = cell.get().expect("Data cell not initialized");
    let data = data_cell.lock().unwrap();
    f(&data)
}




// Hämta en specifik variabel
//let current_dmg = crate::read_collected_data(|data| data.totaldamage);

// Eller skriv ut direkt
//crate::read_collected_data(|data| {println!("Nuvarande skada: {}, Dödade: {}", data.totaldamage, data.numberofkills);});







fn showbanner()
{
        println!("╔═══════════════════════════════════════════════════════════════════════════════════════╗");
        println!("║ Duxes File matching explorer for EU                  v. {}                    [2026]║", VERSION);
        println!("╚═══════════════════════════════════════════════════════════════════════════════════════╝");
}



//fn main() -> std::io::Result<()> {
fn main() -> Result<(), Box<dyn std::error::Error>> {

    showbanner();

    // Anropa funktionen
    inifilereader::load_config_file()?;


    // Nu är CONFIG satt, du kan hämta data så här:
    let config = crate::CONFIG.get().unwrap();
    //println!("{:?}", config.target_file);


    // Create a mutable instance OUTSIDE the loop
    // 1. FIRST: Initialize and set the global variable
    let data = CollectedData::default();
    COLLECTEDDATA.set(Mutex::new(data)).expect("Failed to initialize COLLECTEDDATA");


    // Use expect to handle the Result explicitly:
    //create_mock_log_file(&config.target_file).expect("Failed to create mock log file");



    // Verify the file exists before passing it to the tailer
    if !std::path::Path::new(&config.target_file.clone()).exists() {
        println!("Waiting for target file to be created: {}", config.target_file);
        // Add a loop here to wait for the file if needed
    }

    // 3. Initialize the tailer
    let mut tailer = LineTailer::new(config.target_file.clone())?;




    loop {
        if let Some(line) = tailer.read_next_new_line()?
        {
            //println!("Fångade rad: {}", line);

            // Skydda mot krasch
            let result = std::panic::catch_unwind(|| {
                findpatterns(&line);
            });

            /*match result {
                Ok(_) => println!("findpatterns kördes klart utan krasch."),
                Err(e) => eprintln!("KRASCH i findpatterns: {:?}", e),
            }*/
        }
        std::thread::sleep(std::time::Duration::from_millis(1));

    }

}


