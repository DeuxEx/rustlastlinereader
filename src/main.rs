use nix::sys::stat::Mode;
use nix::unistd::mkfifo;
use notify_debouncer_full::{new_debouncer,notify::{EventKind, RecursiveMode},};
use std::{array, fs::{File, OpenOptions}};
use std::io::{BufRead, BufReader, Seek, SeekFrom, Write};
use std::path::Path;
use std::sync::mpsc::channel;
use std::time::Duration;



static TARGET_FILE: &str = "/home/void/.local/share/Steam/steamapps/compatdata/3642750/pfx/drive_c/users/steamuser/Documents/Entropia Universe/chat.log";
static FIFO_PIPE: &str = "/tmp/min_pipe"; // Variabeln för sökvägen till pipen
static AVATAR: &str = "Deux Pelleman Ex"; // Avatarnamnet

const BLOCKMATCH: &'static [&'static str] = &["#calytrade","#trade","#arktrade"];




/// Söker igenom raden efter specifika mönster och avatarnamn
fn findpatterns(line: &str) {

    // 1. Sök efter avatarnamn
        if line.contains(AVATAR) {println!(" MATCH: Avatar '{}' found!", AVATAR);}

    // 2026-08-07 15:26:23 [#calytrade] [Joshua Crone Craftson] WTB [Entropia Unreal Token] @ 12 ped each

    //Systemevent
    if line.contains("[System]")
    {
        // [System] [] You inflicted 105.0 points of damage
        // 2026-08-07 15:25:10 [System] [] You received Enhanced Adaptive Fuse x (6) Value: 7.02 PED
        // 2026-08-07 15:26:00 [System] [] Critical hit - Additional damage! You inflicted 281.9 points of damage
        // 2026-08-07 15:26:06 [System] [] You have gained 0.0041 experience in your Wounding skill
        if line.contains("You inflicted") || line.contains("Critical hit - Additional damage! You inflicted ")
        {
            //output damage
        }
    }

    //Globalevent
    if line.contains("[Global]")
    {
        // [Globals] [] Deux Pelleman Ex killed a creature (Araneatrox Prowler) with a value of 120 PED!
    }

    //Blocked entries


}



/// Create a named pipe (FIFO) if not exists
fn ensure_fifo_exists(pipe_path: &str) -> std::io::Result<()> {
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
    println!("{}", data);

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



/*
fn formatstring(line: &str) -> String
{
    let mut newline: String;

    //inledning: 2026-08-07 15:25:36 []
    let newline = line.split_at_checked(20);

    //ersätt och-tecknet
    if line.contains("&quot;"){let newline = line.replace("&quot;","'");}

    return newline;
}
*/



fn main() -> Result<(), Box<dyn std::error::Error>> {

    let mut newstring: String;

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

                            //let newstring = formatstring(&last_line);
                            //analyzestring(&newstring);
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
