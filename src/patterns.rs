#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_assignments)]
#![allow(unused)]


use crate::CONFIG;

use crate::CollectedData;
use crate::COLLECTEDDATA;

use crate::update_collected_data;


use colored::*;

use std::i32;
use std::sync::Mutex;
use serde::Deserialize;

use regex::Regex;



//static AVATAR: &str = "Deux Pelleman Ex"; // Avatarnamnet
//const BLOCKMATCH: &'static [&'static str] = &["#calytrade", "#trade", "#arktrade", "Rookie"];



pub fn kor_analys() {
    let config = CONFIG.get().unwrap();

    // Open the file dynamically using the runtime path
    if let Ok(content) = std::fs::read_to_string(&config.target_file)
    {
        println!("Successfully read {} bytes", content.len());
    }

    println!("Starting analysis for file: {}", config.target_file);
    println!("FIFO-pipe in use: {}", config.fifo_pipe);
}



pub fn findpatterns(line: &str) {

    let avatarname = {
        let config_data = CONFIG.get().expect("Config is not initialized!");
        let config = config_data;
        config.avatarname.clone() // Klona ut det du behöver som en ägd sträng
    }; // <-- låset släpps automatiskt här




    {
        let collected_data = COLLECTEDDATA.get().expect("CollectedData is not initialized!");
        let mut data = collected_data.lock().unwrap();

        // Gör dina ändringar på data här...
        // data.some_field = ...
    }; // <-- Låset för COLLECTEDDATA släpps AUTOMATISKT här

    // 3. Fortsätt arbeta med variablerna utan att några lås blockerar tråden


    /*
     * cr*ate::update_collected_data(|data| {data.totaldamage += 10.0;});
     */

    println!("{}",line);




    // 1. Sök efter avatarnamn
    if line.contains(&avatarname)
    {
        println!("MATCH: Avatar '{}' found!", avatarname.green().bold());
        let newstring = formatstring(line);
        println!("{}",newstring.green().bold());
    }



/*
    //Systemevent
    if line.contains("[System]")
    {
        // 2026-08-07 15:26:00 [System] [] Critical hit - Additional damage! You inflicted 281.9 points of damage
        if line.contains("You inflicted")
        {
            let newstring = formatstring(line);
            println!("{}",newstring);

            if let Some((_, after_inflicted)) = line.split_once("You inflicted ")
            {
                if let Some((between, _)) = after_inflicted.split_once("points")
                {
                    if let Ok(damage) = between.trim().parse::<f32>()
                        {
                            data.totaldamage += damage;
                            data.lastdamage = damage;
                            data.totalshots += 1;
                            data.lastmobshots += 1;
                            println!("Current hit: {:.1} | Total so far: {:.1}", data.lastdamage, data.totaldamage);
                        }
                }
            }
        }
*/


/*

        // 2026-08-07 15:25:10 [System] [] You received Enhanced Adaptive Fuse x (6) Value: 7.02 PED
        if line.contains("You received") {

            let newstring = formatstring(line);
            println!("{}",newstring.green().bold());

            // Söker efter ett decimaltal direkt följt av (eller nära) "PED"
            let searchstring = Regex::new(r"(\d+\.\d+)\s*PED").unwrap();

            if let Some(captures) = searchstring.captures(line) {
                // Hämta själva talet (första capture-gruppen)
                if let Some(matched) = captures.get(1) {
                    if let Ok(number) = matched.as_str().parse::<f32>() {
                        data.totallootvalue += number;
                        data.lastlootvalue = number;

                        println!("Current ped: {:.2} | Total so far: {:.2}", data.lastlootvalue, data.totallootvalue);
                        if(data.lastmobshots >0)
                            {
                                data.numberofkills += 1;
                                println!("----------------------------------");
                                println!("Shots on last mob: {}", data.lastmobshots);
                                println!("Number of kills: {}", data.numberofkills);
                                println!("Killcost: {} PED",data.lastmobshots*(1000/config.ammoburn));
                                println!("----------------------------------");
                            }

                        //i samband med detta så resettar vi lastlootvalue så vi får en hyfsad sann bild av mob_cost_to_kill
                        //det kan slå på några loot-rader men killcosten borde blir ganska exakt, det är lootvärdet som kan slå lite.
                        data.lastlootvalue = 0.0;
                        data.lastdamage = 0.0;
                        data.lastmobshots = 0;
                    }
                }
            }
        }

        // 2026-08-07 15:26:06 [System] [] You have gained 0.0041 experience in your Wounding skill
    }
*/


    //Globalevent
    if line.contains("[Global]")
    {
        // [Globals] [] Deux Pelleman Ex killed a creature (Araneatrox Prowler) with a value of 120 PED!

        let newstring = formatstring(line);
        println!("{}",newstring);
    }

return;

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


    if newline.contains("&quot;")
    {
        newline = newline.replace("&quot;", "\"");
    }


    if newline.contains("&lt;")
    {
        newline = newline.replace("&lt;", "<");
    }



    if newline.contains("&gt;")
    {
        newline = newline.replace("&gt;", ">");
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






