#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]


use std::{array, fs::{File, OpenOptions}};
use std::io::{BufRead, BufReader, Seek, SeekFrom, Write};
use std::path::Path;





/// Läser den sista icke-tomma raden ur en fil genom att söka sig bakåt från slutet
pub fn read_last_line<P: AsRef<Path>>(path: P) -> std::io::Result<Option<String>> {
    let mut file = File::open(path)?;
    let file_size = file.metadata()?.len();

    if file_size == 0 {
        return Ok(None);
    }

    let read_size = file_size.min(1024) as i64;
    file.seek(SeekFrom::End(-read_size))?;

    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().collect::<Result<_, _>>()?;

    Ok(lines.into_iter().rev().find(|line| !line.trim().is_empty()))
}
