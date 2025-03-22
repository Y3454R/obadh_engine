use obadh_engine::ObadhEngine;

fn main() {
    let engine = ObadhEngine::new();
    
    println!("===== BASIC TEST CASES =====");
    println!("bhalo => {}", engine.transliterate("bhalo"));
    println!("bhalO => {}", engine.transliterate("bhalO"));
    println!("kok => {}", engine.transliterate("kok"));
    println!("kOk => {}", engine.transliterate("kOk"));
    println!("v => {}", engine.transliterate("v"));
    println!("boi => {}", engine.transliterate("boi"));
    println!("kokko => {}", engine.transliterate("kokko"));
    println!("kokkO => {}", engine.transliterate("kokkO"));
    println!("kk => {}", engine.transliterate("kk"));
    println!("t`` => {}", engine.transliterate("t``"));
    println!("kOnnO => {}", engine.transliterate("kOnnO"));
    
    println!("\n===== YAPHALA TEST CASES =====");
    println!("biddaloy => {}", engine.transliterate("biddaloy"));
    println!("bidyaloy => {}", engine.transliterate("bidyaloy"));
    println!("madhyam => {}", engine.transliterate("madhyam"));
    println!("adhyapon => {}", engine.transliterate("adhyapon"));
    println!("byakti => {}", engine.transliterate("byakti"));
    println!("shOmpadyo => {}", engine.transliterate("shOmpadyo"));
    println!("madhyOm => {}", engine.transliterate("madhyOm"));
    println!("vidyut`` => {}", engine.transliterate("vidyut``"));
    
    println!("\n===== COMPLEX CONJUNCTS TEST CASES =====");
    println!("ShTho => {}", engine.transliterate("ShTho"));
    println!("ddho => {}", engine.transliterate("ddho"));
    println!("tmo => {}", engine.transliterate("tmo"));
    println!("lko => {}", engine.transliterate("lko"));
    println!("ngo => {}", engine.transliterate("ngo"));
    println!("ntro => {}", engine.transliterate("ntro"));
    println!("kkhmo => {}", engine.transliterate("kkhmo"));
    println!("ttwo => {}", engine.transliterate("ttwo"));
    println!("ddo => {}", engine.transliterate("ddo"));
    println!("Tro => {}", engine.transliterate("Tro"));
    println!("Dro => {}", engine.transliterate("Dro"));
    
    println!("\n===== BOFOLA TEST CASES =====");
    println!("bishwo => {}", engine.transliterate("bishwo"));
    println!("dwip => {}", engine.transliterate("dwip"));
    println!("atmbiSwas => {}", engine.transliterate("atmbiSwas"));
    println!("dwIp => {}", engine.transliterate("dwIp"));
    println!("dhwoni => {}", engine.transliterate("dhwoni"));
    println!("anubadwok => {}", engine.transliterate("anubadwok"));
} 