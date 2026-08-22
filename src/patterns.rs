#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_assignments)]
#![allow(unused)]


use crate::{CONFIG, filewatching::LineTailer}; // Ger tillgång till den globala CONFIG

//ansi colors and details on text
use colored::*;
//use nix::libc::int32_t;
use std::i32;
use serde::Deserialize;


use crate::COLLECTEDDATA;


static AVATAR: &str = "Deux Pelleman Ex"; // Avatarnamnet
const BLOCKMATCH: &'static [&'static str] = &["#calytrade", "#trade", "#arktrade", "#Rookie"];




#[derive(Debug, Default)]
pub struct CollectedData {
    pub totaldamage: f32,
    pub lastdamage: f32,
    pub totallootvalue: f32,
    pub lastlootvalue: f32,
    //pub totalhealpoints: f32,
    //pub lasthealpoints: f32,
}



pub fn kor_analys() {
    let conf = CONFIG.get().unwrap();

    println!("Starting analysis for file: {}", conf.target_file);
    println!("FIFO-pipe in use: {}", conf.fifo_pipe);
}



/// Söker igenom raden efter specifika mönster och avatarnamn
pub fn findpatterns(line: &str) {

    // 1. Create a mutable instance OUTSIDE the loop
    let mut data = CollectedData::default();



    //Blocked entries
    for blocks in BLOCKMATCH
    {
        if line.contains(blocks)
        {
            // 2026-08-07 15:26:23 [#calytrade] [Joshua Crone Craftson] WTB [Entropia Unreal Token] @ 12 ped each
            println!("block found, skipping to next line: {}", blocks.red().bold());
            return;
        }
    }

    // 1. Sök efter avatarnamn
    if line.contains(AVATAR)
    {
        //println!(" MATCH: Avatar '{}' found!", AVATAR.green().bold());
        let newstring = formatstring(line);
        println!("{}",newstring);
        return;
    }


    //Systemevent
    if line.contains("[System]")
    {
        // 2026-08-07 15:26:00 [System] [] Critical hit - Additional damage! You inflicted 281.9 points of damage
        if line.contains("You inflicted")
        {
            // Extrahera decimaltalet direkt mellan "inflicted" och "points"
            if let Some((_, after_inflicted)) = line.split_once("inflicted")
            {
                if let Some((between, _)) = after_inflicted.split_once("points")
                {
                    if let Ok(damage) = between.trim().parse::<f32>()
                    {
                        // Här har du ditt decimaltal i variabeln `damage` (f64)
                        //println!("Skada: {}", damage);

                        let incoming_hits: f32 = damage;


                        // 3. Mutate the struct fields directly
                        data.totaldamage += damage;
                        data.lastdamage = damage;

                        println!("Current hit: {:.1} | Total so far: {:.1}", data.lastdamage, data.totaldamage);
                    }
                }
            }
        }


        // 2026-08-07 15:25:10 [System] [] You received Enhanced Adaptive Fuse x (6) Value: 7.02 PED
        // 2026-08-07 15:26:06 [System] [] You have gained 0.0041 experience in your Wounding skill
        if line.contains("You received")
        {
            // Extrahera decimaltalet direkt mellan "received" och "PED"
            if let Some((_, after_received)) = line.split_once("received")
            {
                if let Some((between, _)) = after_received.split_once("PED")
                {
                    if let Ok(pedreceived) = between.trim().parse::<f32>()
                    {
                        // Här har du ditt decimaltal i variabeln `damage` (f64)
                        //println!("Skada: {}", damage);

                        let incoming_peds: f32 = pedreceived;

                        // 3. Mutate the struct fields directly
                        data.totallootvalue += pedreceived;
                        data.lastlootvalue = pedreceived;

                        println!("Current ped: {:.1} | Total so far: {:.1}", data.lastlootvalue, data.totallootvalue);
                    }
                }
            }
        }
    }


    //Globalevent
    if line.contains("[Global]") {
        // [Globals] [] Deux Pelleman Ex killed a creature (Araneatrox Prowler) with a value of 120 PED!

        let newstring = formatstring(line);
        println!("{}",newstring);
        return;
    }
}



// Analyserar strängen, skriver ut den och skickar vidare till en pipe
//fn analyzestring(data: &str, pipe_path: &str) {
pub fn analyzestring(data: &str) {
    //println!("Standard utskrift: {}", data);
    //println!("{}", data);

    let newstring = formatstring(data);
    println!("{}",newstring);

    // Kör mönstersökningen på den inkomna raden
    findpatterns(data);

    //if let Err(e) = send_to_pipe(data, pipe_path) {
    //    eprintln!("Kunde inte skriva till pipe {}: {}", pipe_path, e);
    //}
}




pub fn formatstring(line: &str) -> String
{
    // 1. Skapa en muterbar (föränderlig) String från input
    let mut newline = line.to_string();

    // 2. Ersätt text direkt i vår föränderliga 'newline'
    if newline.contains("&quot;")
    {
        newline = newline.replace("&quot;", "'");
    }

    // .split_off(20) returnerar allt FRÅN tecken 20 och framåt.
    // Vi sparar den delen i newline och kastar därmed bort de första 20.
    if newline.len() >= 20
    {
        newline = newline.split_off(20);
    }

    // 4. Returnera det färdiga resultatet
    return newline;
}






