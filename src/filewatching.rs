#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]


use crate::analyzestring;
use crate::exists;

use std::fs::File;
use std::io::{BufRead, BufReader, Seek, SeekFrom};
use std::path::{Path, PathBuf};


pub struct LineTailer {
    path: PathBuf,
    last_pos: u64,
}


use colored::Colorize;
use nix::libc::file_handle;
use notify_debouncer_full::{
    new_debouncer,
    notify::{EventKind, RecursiveMode, event::ModifyKind},
};

use serde_ini::ser::Error;





impl LineTailer {
    pub fn new<P: AsRef<Path>>(path: P) -> std::io::Result<Self> {
        let path = path.as_ref().to_path_buf();
        // Ställ in startpositionen på slutet av filen direkt vid start
        let mut file = File::open(&path)?;
        let last_pos = file.seek(SeekFrom::End(0))?;

        Ok(Self { path, last_pos })
    }

    pub fn read_next_new_line(&mut self) -> std::io::Result<Option<String>> {
        let mut file = File::open(&self.path)?;
        let current_len = file.metadata()?.len();

        // SKRIV UT FÖR VARJE VARV:
        //println!("DEBUG: current_len={}, last_pos={}", current_len, self.last_pos);

        if current_len < self.last_pos {
            self.last_pos = 0;
        } else if current_len == self.last_pos {
            return Ok(None);
        }

        file.seek(SeekFrom::Start(self.last_pos))?;
        let mut reader = BufReader::new(file);

        let mut line = String::new();
        let bytes_read = reader.read_line(&mut line)?;

        //println!("DEBUG: bytes_read={}, line={:?}", bytes_read, line);

        if bytes_read > 0 {
            self.last_pos += bytes_read as u64;

            if line.ends_with("\r\n") {
                line.truncate(line.len() - 2);
            } else if line.ends_with('\n') {
                line.truncate(line.len() - 1);
            }

            Ok(Some(line))
        } else {
            Ok(None)
        }
    }


}
