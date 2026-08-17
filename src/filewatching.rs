#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]


use crate::analyzestring;
use crate::exists;

use std::fs::File;
use std::io::{BufRead, BufReader, Seek, SeekFrom};
use std::path::Path;

pub struct LineTailer {
    reader: BufReader<File>,
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
    /// Skapar en ny tailer och ställer sig i slutet av filen
    pub fn new<P: AsRef<Path>>(path: P) -> std::io::Result<Self> {
        let mut file = File::open(path)?;
        let last_pos = file.seek(SeekFrom::End(0))?;
        Ok(Self {
            reader: BufReader::new(file),
           last_pos,
        })
    }

    /// Läser nästa helt nya rad om filen har vuxit.
    /// Returnerar `Ok(None)` om inga nya färdiga rader finns ännu.
    pub fn read_next_new_line(&mut self) -> std::io::Result<Option<String>> {
        let current_len = self.reader.get_ref().metadata()?.len();

        // Hantera om filen trunkerats/roterats
        if current_len < self.last_pos {
            self.last_pos = 0;
            self.reader.seek(SeekFrom::Start(0))?;
        } else if current_len == self.last_pos {
            return Ok(None); // Inga nya data
        }

        let mut line = String::new();
        let bytes_read = self.reader.read_line(&mut line)?;

        // Säkerställ att raden har en radbrytning (är helt färdigskriven)
        if bytes_read > 0 && line.ends_with('\n') {
            self.last_pos += bytes_read as u64;

            // Ta bort radbrytningen i slutet
            if line.ends_with("\r\n") {
                line.truncate(line.len() - 2);
            } else {
                line.truncate(line.len() - 1);
            }

            Ok(Some(line))
        } else {
            // Raden är inte färdigskriven än (partiell skrivning)
            // Backa tillbaka så vi kan läsa om den när hela raden finns
            self.reader.seek(SeekFrom::Start(self.last_pos))?;
            Ok(None)
        }
    }
}
