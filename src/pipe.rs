#![allow(dead_code)]

use std::path::Path;
use nix::sys::stat::Mode;
use nix::unistd::mkfifo;
use std::{array, fs::{File, OpenOptions}};
use std::io::{BufRead, BufReader, Seek, SeekFrom, Write};


/// Create a named pipe (FIFO) if not exists
pub fn ensure_fifo_exists(pipe_path: &str) -> std::io::Result<()> {
    let path = Path::new(pipe_path);

    if !path.exists()
    {
        // Skapa FIFO med rättigheterna 0666 (läs/skriv för alla, modifieras av umask)
        let mode = Mode::S_IRUSR | Mode::S_IWUSR | Mode::S_IRGRP | Mode::S_IWGRP | Mode::S_IROTH | Mode::S_IWOTH;

        match mkfifo(path, mode)
        {
            Ok(_) => println!("Created pipe: {}", pipe_path),
            Err(nix::errno::Errno::EEXIST) => {} // Finns redan, ingenting behöver göras
            Err(e) => return Err(std::io::Error::new(std::io::ErrorKind::Other, e)),
        }
    }
    else
    {
        println!("PIPE already exists {}", pipe_path);
    }
    Ok(())
}




/// Skriver raden till pipen
pub fn send_to_pipe(data: &str, pipe_path: &str) -> std::io::Result<()> {
    let mut pipe = OpenOptions::new().write(true).open(pipe_path)?;
    writeln!(pipe, "{}", data)?;
    pipe.flush()?;
    Ok(())
}

