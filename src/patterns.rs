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
        config.avatarname.clone()
    }; // <-- låset släpps automatiskt här

    let ammoburn = {
        let config_data = CONFIG.get().expect("Config is not initialized!");
        let config = config_data;
        config.ammoburn.clone()
    }; // <-- låset släpps automatiskt här


    let mut totaldamage: f32 = 0.0;
/*    let mut totaldamage = {
        let collected_data = COLLECTEDDATA.get().expect("CollectedData is not initialized!");
        let mut data = collected_data.lock().unwrap();
        data.totaldamage.clone()
    }; // <-- Låset för COLLECTEDDATA släpps AUTOMATISKT här
*/

    let mut lastdamage: f32 = 0.0;
/*    let mut lastdamage = {
        let collected_data = COLLECTEDDATA.get().expect("CollectedData is not initialized!");
        let mut data = collected_data.lock().unwrap();
        data.lastdamage.clone()
    }; // <-- Låset för COLLECTEDDATA släpps AUTOMATISKT här
*/

    let mut totalshots: i32 = 0;
/*    let mut totalshots = {
        let collected_data = COLLECTEDDATA.get().expect("CollectedData is not initialized!");
        let mut data = collected_data.lock().unwrap();
        data.totalshots.clone()
    }; // <-- Låset för COLLECTEDDATA släpps AUTOMATISKT här
*/

    let mut lastmobshots: i32 = 0;
/*    let mut lastmobshots = {
        let collected_data = COLLECTEDDATA.get().expect("CollectedData is not initialized!");
        let mut data = collected_data.lock().unwrap();
        data.lastmobshots.clone()
    }; // <-- Låset för COLLECTEDDATA släpps AUTOMATISKT här
*/

    let mut numberofkills: i32 = 0;
/*    let mut numberofkills = {
        let collected_data = COLLECTEDDATA.get().expect("CollectedData is not initialized!");
        let mut data = collected_data.lock().unwrap();
        data.numberofkills.clone()
    }; // <-- Låset för COLLECTEDDATA släpps AUTOMATISKT här
*/

    let mut lastlootvalue: f32 = 0.0;
/*    let mut lastlootvalue = {
        let collected_data = COLLECTEDDATA.get().expect("CollectedData is not initialized!");
        let mut data = collected_data.lock().unwrap();
        data.lastlootvalue.clone()
    }; // <-- Låset för COLLECTEDDATA släpps AUTOMATISKT här
*/

    let mut totallootvalue: f32 = 0.0;
/*    let mut totallootvalue = {
        let collected_data = COLLECTEDDATA.get().expect("CollectedData is not initialized!");
        let mut data = collected_data.lock().unwrap();
        data.totallootvalue.clone()
    }; // <-- Låset för COLLECTEDDATA släpps AUTOMATISKT här
*/



    println!("{}",line);




    // 1. Sök efter avatarnamn
    if line.contains(&avatarname)
    {
        println!("MATCH: Avatar '{}' found!", avatarname.green().bold());
        let newstring = formatstring(line);
        println!("{}",newstring.green().bold());
    }




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
                        totaldamage += damage;
                        lastdamage = damage;
                        totalshots += 1;
                        lastmobshots += 1;

                        crate::update_collected_data(|data|
                        {
                            data.totaldamage = totaldamage;
                            data.lastdamage = lastdamage;
                            data.totalshots += 1;
                            data.lastmobshots += 1;
                        });

                        //println!("Current hit: {:.1} | Total so far: {:.1}", data.lastdamage, data.totaldamage);
                    }
                }
            }
        }
    }




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

                        //kolla efter hur mycket ammo som skjutits så vi inte får falsk data när det kommer flera rader med loot.
                        if(lastmobshots >0)
                            {
                                numberofkills += 1;
                                crate::update_collected_data(|data|
                                {
                                    data.numberofkills += 1;
                                    data.lastmobshots += 1;
                                });

                                println!("----------------------------------");
                                println!("Shots on last mob: {}", lastmobshots);
                                println!("Number of kills: {}", numberofkills);
                                println!("Killcost: {} PED",lastmobshots*(1000/ammoburn));
                                println!("----------------------------------");


                                // Hämta en specifik variabel
                                let current_dmg = crate::read_collected_data(|data| data.totaldamage);

                                // Eller skriv ut direkt
                                crate::read_collected_data(|data| {println!("Nuvarande skada: {}, Dödade: {}", data.totaldamage, data.numberofkills);});

                                //println!("Current ped: {:.2} | Total so far: {:.2}", data.lastlootvalue, data.totallootvalue);

                            }

                        //i samband med detta så resettar vi lastlootvalue så vi får en hyfsad sann bild av mob_cost_to_kill
                        //det kan slå på några loot-rader men killcosten borde blir ganska exakt, det är lootvärdet som kan slå lite.

                        crate::update_collected_data(|data| {data.totallootvalue += number;});
                        crate::update_collected_data(|data| {data.lastlootvalue = number;});

                        crate::update_collected_data(|data| {data.lastlootvalue = 0.0;});
                        crate::update_collected_data(|data| {data.lastdamage = 0.0;});
                        crate::update_collected_data(|data| {data.lastmobshots = 0;});

                    }
                }
            }
        }

        // 2026-08-07 15:26:06 [System] [] You have gained 0.0041 experience in your Wounding skill



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






