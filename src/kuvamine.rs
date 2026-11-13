//! Tulemuste kuvamine terminalis.

use crate::analuus::{Analuus, Soovitus};
use colored::{ColoredString, Colorize};

pub fn prindi_pais(kurss: Option<f64>) {
    println!();
    println!("{}", "VÄÄRISMETALLIDE HINNAD JA SOOVITUSED".bold().yellow());
    println!("{}", "Hinnad: troy untsi kohta (viimased ~6 kuud)".dimmed());
    if let Some(k) = kurss {
        println!("{}", format!("Kurss: 1 EUR = {:.4} USD", k).dimmed());
    }
    println!();
}

pub fn prindi_metall(nimi: &str, a: &Analuus, kurss: Option<f64>) {
    println!("{}", format!("=== {} ===", nimi).bold().cyan());
    println!("  Hind:            {}", hind_tekst(Some(a.hind), kurss));
    println!("  Muutus 1 päev:   {}", muutus_tekst(a.muutus_1p));
    println!("  Muutus 30 päeva: {}", muutus_tekst(a.muutus_30p));
    println!("  Keskmine 20p:    {}", hind_tekst(a.sma20, kurss));
    println!("  Keskmine 50p:    {}", hind_tekst(a.sma50, kurss));
    println!("  RSI (14):        {}", arv_tekst(a.rsi));
    println!(
        "  Asukoht vahemikus: {:.0}% (0% = poole aasta madalaim, 100% = kõrgeim)",
        a.vahemiku_asukoht * 100.0
    );
    println!("  Punktid:         {:+}", a.punktid);
    println!("  Soovitus:        {}", soovituse_tekst(&a.soovitus));

    println!("  Põhjendus:");
    for p in &a.pohjendused {
        println!("    - {}", p);
    }
    println!();
}

pub fn prindi_hoiatus() {
    println!(
        "{}",
        "NB! See on lihtne õppeprogramm, mitte finantsnõu. Ajalugu ei garanteeri tulevikku."
            .italic()
            .dimmed()
    );
}

/// Näitab hinna USD-des ja (kui kurss on olemas) ka eurodes.
fn hind_tekst(usd: Option<f64>, kurss: Option<f64>) -> String {
    match (usd, kurss) {
        (Some(u), Some(k)) => format!("{:.2} USD  ({:.2} EUR)", u, u / k),
        (Some(u), None) => format!("{:.2} USD", u),
        (None, _) => "puudub".to_string(),
    }
}

/// Muudab protsendi värviliseks tekstiks (roheline = tõus, punane = langus).
fn muutus_tekst(muutus: Option<f64>) -> ColoredString {
    match muutus {
        Some(p) if p >= 0.0 => format!("{:+.2}%", p).green(),
        Some(p) => format!("{:+.2}%", p).red(),
        None => "puudub".normal(),
    }
}

fn arv_tekst(arv: Option<f64>) -> String {
    match arv {
        Some(v) => format!("{:.2}", v),
        None => "puudub".to_string(),
    }
}

fn soovituse_tekst(s: &Soovitus) -> ColoredString {
    match s {
        Soovitus::TugevOst => "TUGEV OSTUSIGNAAL – hea aeg varuda".green().bold(),
        Soovitus::Ost => "OSTA – hind tundub pigem soodne".green(),
        Soovitus::Oota => "OOTA – selget signaali pole".yellow(),
        Soovitus::Muuk => "MÜÜ – hind tundub pigem kallis".red(),
        Soovitus::TugevMuuk => "TUGEV MÜÜGISIGNAAL – hea aeg kasumit võtta".red().bold(),
    }
}
