#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]


static AVATAR: &str = "Deux Pelleman Ex"; // Avatarnamnet
const BLOCKMATCH: &'static [&'static str] = &["#calytrade","#trade","#arktrade"];




/// Söker igenom raden efter specifika mönster och avatarnamn
pub fn findpatterns(line: &str) {

    //Blocked entries
    for blocks in BLOCKMATCH
    {
        if line.contains(blocks)
        {
            // 2026-08-07 15:26:23 [#calytrade] [Joshua Crone Craftson] WTB [Entropia Unreal Token] @ 12 ped each
            println!("block found, skipping to next line: {}",blocks);
            return;
        }
    }


    // 1. Sök efter avatarnamn
        if line.contains(AVATAR) {println!(" MATCH: Avatar '{}' found!", AVATAR);}


    //Systemevent
    if line.contains("[System]")
    {
        // [System] [] You inflicted 105.0 points of damage
        // 2026-08-07 15:25:10 [System] [] You received Enhanced Adaptive Fuse x (6) Value: 7.02 PED
        // 2026-08-07 15:26:00 [System] [] Critical hit - Additional damage! You inflicted 281.9 points of damage
        // 2026-08-07 15:26:06 [System] [] You have gained 0.0041 experience in your Wounding skill
        if line.contains("You inflicted") || line.contains("Critical hit - Additional damage! You inflicted ")
        {
            // Extrahera decimaltalet direkt mellan "inflicted" och "points"
            if let Some((_, after_inflicted)) = line.split_once("inflicted") {
                if let Some((between, _)) = after_inflicted.split_once("points") {
                    if let Ok(damage) = between.trim().parse::<f64>() {
                        // Här har du ditt decimaltal i variabeln `damage` (f64)
                        println!(" EXTRAHERAD SKADA: {}", damage);
                    }
                }
            }
        }
    }

    //Globalevent
    if line.contains("[Global]")
    {
        // [Globals] [] Deux Pelleman Ex killed a creature (Araneatrox Prowler) with a value of 120 PED!
    }
}







// Analyserar strängen, skriver ut den och skickar vidare till en pipe
//fn analyzestring(data: &str, pipe_path: &str) {
pub fn analyzestring(data: &str) {
    //println!("Standard utskrift: {}", data);
    println!("{}", data);

    // Kör mönstersökningen på den inkomna raden
    findpatterns(data);

    //if let Err(e) = send_to_pipe(data, pipe_path) {
    //    eprintln!("Kunde inte skriva till pipe {}: {}", pipe_path, e);
    //}
}




/*
 * pub fn formatstring(line: &str) -> String
 * {
 *    let mut newline: String;
 *
 *    //inledning: 2026-08-07 15:25:36 []
 *    let newline = line.split_at_checked(20);
 *
 *    //ersätt och-tecknet
 *    if line.contains("&quot;"){let newline = line.replace("&quot;","'");}
 *
 *    return newline;
 * }
 */

