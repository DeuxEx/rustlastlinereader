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
    file: File,
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
    pub fn new<P: AsRef<Path>>(path: P) -> std::io::Result<Self> {
        let path = path.as_ref().to_path_buf();
        let mut file = File::open(&path)?;

        // Starta från slutet av filen
        let last_pos = file.seek(SeekFrom::End(0))?;
        let reader = BufReader::new(file.try_clone()?);

        Ok(Self {
            path,
            file,
            reader,
            last_pos,
        })
    }

    /// Läser ALLA nya kompletta rader som tillkommit sedan förra anropet.
    pub fn read_new_lines(&mut self) -> std::io::Result<Vec<String>> {
        // Kontrollera om filen har roterats eller tömts (truncated)
        let current_len = self.file.metadata()?.len();
        if current_len < self.last_pos {
            self.last_pos = 0;
            self.file.seek(SeekFrom::Start(0))?;
            self.reader = BufReader::new(self.file.try_clone()?);
        } else if current_len == self.last_pos {
            return Ok(Vec::new());
        }

        let mut lines = Vec::new();

        loop {
            let mut line = String::new();
            let bytes_read = self.reader.read_line(&mut line)?;

            if bytes_read == 0 {
                // Slut på filen (EOF)
                break;
            }

            // Kontrollera om raden faktiskt är färdigskriven (slutar med \n)
            if !line.ends_with('\n') {
                // Raden är inte färdigskriven ännu!
                // Återställ sökläget så vi läser om hela raden nästa gång.
                self.file.seek(SeekFrom::Start(self.last_pos))?;
                self.reader = BufReader::new(self.file.try_clone()?);
                break;
            }

            // Raden är komplett — uppdatera positionen
            self.last_pos += bytes_read as u64;

            // Rensa trailing newlines (\r\n eller \n)
            if line.ends_with("\r\n") {
                line.truncate(line.len() - 2);
            } else if line.ends_with('\n') {
                line.truncate(line.len() - 1);
            }

            lines.push(line);
        }

        Ok(lines)
    }
}
