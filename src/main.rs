use nix::sys::stat::Mode;
use nix::unistd::mkfifo;
use notify_debouncer_full::{new_debouncer,notify::{EventKind, RecursiveMode},};
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Seek, SeekFrom, Write};
use std::path::Path;
use std::sync::mpsc::channel;
use std::time::Duration;

static TARGET_FILE: &str = "/home/void/.local/share/Steam/steamapps/compatdata/3642750/pfx/drive_c/users/steamuser/Documents/Entropia Universe/chat.log";
static FIFO_PIPE: &str = "/tmp/min_pipe"; // Variabeln för sökvägen till din pipe
static AVATARS: &str = "Deux Pelleman Ex";
//static WELCOME_TEXT: &str = "Hello";



/// Söker igenom raden efter specifika mönster och avatarnamn
fn findpatterns(line: &str) {
    // Lista på avatarnamn att leta efter

    // 1. Sök efter avatarnamn
    //for name in &avatars {
        if line.contains(AVATARS) {
            println!(" MATCH: Hittade avatar '{}' i raden!", AVATARS);
        }
    //}

    // 2. Här kan du lägga till fler mönstersökningar (t.ex. med Regex eller vanliga strängjämförelser)
    if line.contains("ERROR") {
        println!(" MÖNSTER: Upptäckte ett felmeddelande i raden!");
    }
}



/// Skapar en named pipe (FIFO) om den inte redan finns
fn ensure_fifo_exists(pipe_path: &str) -> std::io::Result<()> {
    let path = Path::new(pipe_path);

    if !path.exists()
    {
        // Skapa FIFO med rättigheterna 0666 (läs/skriv för alla, modifieras av umask)
        let mode = Mode::S_IRUSR | Mode::S_IWUSR | Mode::S_IRGRP | Mode::S_IWGRP | Mode::S_IROTH | Mode::S_IWOTH;

        match mkfifo(path, mode)
        {
            Ok(_) => println!("Skapade named pipe: {}", pipe_path),
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



/// Läser den sista icke-tomma raden ur en fil genom att söka sig bakåt från slutet
fn read_last_line<P: AsRef<Path>>(path: P) -> std::io::Result<Option<String>> {
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



// Analyserar strängen, skriver ut den och skickar vidare till en pipe
//fn analyzestring(data: &str, pipe_path: &str) {
fn analyzestring(data: &str) {
    //println!("Standard utskrift: {}", data);

    // Kör mönstersökningen på den inkomna raden
    findpatterns(data);

    //if let Err(e) = send_to_pipe(data, pipe_path) {
    //    eprintln!("Kunde inte skriva till pipe {}: {}", pipe_path, e);
    //}
}



/// Skriver raden till pipen
fn send_to_pipe(data: &str, pipe_path: &str) -> std::io::Result<()> {
    let mut pipe = OpenOptions::new().write(true).open(pipe_path)?;
    writeln!(pipe, "{}", data)?;
    pipe.flush()?;
    Ok(())
}



fn main() -> Result<(), Box<dyn std::error::Error>> {

//    let target_file = "/home/void/.local/share/Steam/steamapps/compatdata/3642750/pfx/drive_c/users/steamuser/Documents/Entropia Universe/chat.log";
//    let fifo_pipe = "/tmp/min_pipe"; // Variabeln för sökvägen till din pipe

    // 1. Säkerställ att pipen finns (skapas automatiskt om den saknas)
    ensure_fifo_exists(FIFO_PIPE)?;

    // 2. Säkerställ att målfilen finns
    let path = Path::new(TARGET_FILE);
    if !path.exists() {
        File::create(path)?;
    }

    let (tx, rx) = channel();
    let mut debouncer = new_debouncer(Duration::from_millis(200), None, tx)?;
    debouncer.watch(path, RecursiveMode::NonRecursive)?;

    println!("Watchdog started for: {} (Pipe target: {})",path.display(),FIFO_PIPE);

    let mut last_line: String;

    for result in rx {
        match result {
            Ok(events) => {
                let is_modified = events.iter().any(|event| {
                    matches!(event.kind, EventKind::Modify(_))
                });

                if is_modified {
                    match read_last_line(path) {
                        Ok(Some(line)) => {
                            last_line = line;
                            //analyzestring(&last_line, fifo_pipe);
                            analyzestring(&last_line);
                        }
                        Ok(None) => println!("The file is empty."),
                        Err(e) => eprintln!("Error reading: {}", e),
                    }
                }
            }
            Err(errors) => {
                for err in errors {
                    eprintln!("Debounce error: {:?}", err);
                }
            }
        }
    }

    Ok(())
}
