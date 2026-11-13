//! Hinnakalkulaator.
//!
//! Kasutaja sisestab ainult metalli, kaalu (ja soovi korral proovi),
//! programm arvutab kogu väärtuse praeguse turuhinna järgi.

use crate::andmed::{self, METALLID};
use colored::Colorize;
use std::io::{self, Write};

/// Mitu grammi on ühes troy untsis (väärismetallide hinnad on untsi kohta).
const GRAMMI_UNTSIS: f64 = 31.1034768;

/// Ühikud, milles kasutaja kaalu sisestada saab.
enum Uhik {
    Gramm,
    Kilogramm,
    TroyUnts,
}

impl Uhik {
    /// Mitu grammi on ühes selle ühikus.
    fn grammi(&self) -> f64 {
        match self {
            Uhik::Gramm => 1.0,
            Uhik::Kilogramm => 1000.0,
            Uhik::TroyUnts => GRAMMI_UNTSIS,
        }
    }

    fn nimi(&self) -> &'static str {
        match self {
            Uhik::Gramm => "g",
            Uhik::Kilogramm => "kg",
            Uhik::TroyUnts => "troy unts",
        }
    }
}

/// Arvutab väärtuse.
///   hind_untsi  – metalli hind USD troy untsi kohta
///   kaal_g      – eseme kaal grammides
///   proov       – puhtus promillides (999 = 99,9% puhas, 750 = 18 karaati)
pub fn arvuta_vaartus(hind_untsi: f64, kaal_g: f64, proov: f64) -> f64 {
    let puhta_metalli_g = kaal_g * proov / 1000.0;
    puhta_metalli_g / GRAMMI_UNTSIS * hind_untsi
}

/// Küsib kasutajalt rea teksti.
pub fn loe_rida(kysimus: &str) -> String {
    print!("{}", kysimus);
    io::stdout().flush().ok();
    let mut sisend = String::new();
    io::stdin().read_line(&mut sisend).ok();
    sisend.trim().to_string()
}

/// Küsib positiivse arvu (komaga või punktiga).
fn loe_arv(kysimus: &str) -> Option<f64> {
    loe_rida(kysimus)
        .replace(',', ".")
        .parse::<f64>()
        .ok()
        .filter(|v| *v > 0.0)
}

fn loe_uhik() -> Uhik {
    match loe_rida("Ühik: 1) gramm  2) kilogramm  3) troy unts  (Enter = gramm): ").as_str() {
        "2" => Uhik::Kilogramm,
        "3" => Uhik::TroyUnts,
        _ => Uhik::Gramm,
    }
}

/// Kalkulaatori põhitsükkel.
pub fn kaivita() {
    println!();
    println!("{}", "HINNAKALKULAATOR".bold().yellow());

    // Kui kurssi ei saa, näitame ainult USD
    let kurss = andmed::lae_eur_usd_kurss().ok();

    loop {
        // 1. Metalli valik
        println!("\nVali metall (tühi rida = tagasi menüüsse):");
        for (i, m) in METALLID.iter().enumerate() {
            println!("  {}) {}", i + 1, m.nimi);
        }
        let valik = loe_rida("Number: ");
        if valik.is_empty() {
            return;
        }
        let metall = match valik.parse::<usize>() {
            Ok(n) if n >= 1 && n <= METALLID.len() => &METALLID[n - 1],
            _ => {
                println!("Vigane valik.");
                continue;
            }
        };

        // 2. Kaal
        let uhik = loe_uhik();
        let kaal = match loe_arv(&format!("Kaal ({}): ", uhik.nimi())) {
            Some(v) => v,
            None => {
                println!("Sisesta positiivne arv.");
                continue;
            }
        };
        let kaal_g = kaal * uhik.grammi();

        // 3. Proov (puhtus)
        let proov_tekst = loe_rida("Proov (nt 999, 925, 750, 585; Enter = 999): ");
        let proov = if proov_tekst.is_empty() {
            999.0
        } else {
            match proov_tekst.replace(',', ".").parse::<f64>() {
                Ok(p) if p > 0.0 && p <= 1000.0 => p,
                _ => {
                    println!("Proov peab olema vahemikus 1–1000.");
                    continue;
                }
            }
        };

        // 4. Praegune hind
        let hinnad = match andmed::lae_hinnad(metall.symbol) {
            Ok(h) => h,
            Err(viga) => {
                println!("Hinna laadimine ebaõnnestus ({}).", viga);
                continue;
            }
        };
        let Some(&hind_untsi) = hinnad.last() else {
            println!("Hind puudub.");
            continue;
        };

        // 5. Tulemus
        let kokku_usd = arvuta_vaartus(hind_untsi, kaal_g, proov);
        let puhas_g = kaal_g * proov / 1000.0;
        let hind_grammi_usd = hind_untsi / GRAMMI_UNTSIS;

        println!();
        println!("{}", format!("--- {} ---", metall.nimi).bold().cyan());
        println!("  Kaal:               {:.3} g", kaal_g);
        println!("  Proov:              {:.0} ({:.1}% puhast metalli)", proov, proov / 10.0);
        println!("  Puhta metalli kaal: {:.3} g", puhas_g);
        match kurss {
            Some(k) => {
                println!("  Hind grammi kohta:  {:.2} USD ({:.2} EUR)", hind_grammi_usd, hind_grammi_usd / k);
                println!(
                    "  {} {:.2} USD  ({:.2} EUR)",
                    "LÕPLIK HIND:".bold().green(),
                    kokku_usd,
                    kokku_usd / k
                );
            }
            None => {
                println!("  Hind grammi kohta:  {:.2} USD", hind_grammi_usd);
                println!("  {} {:.2} USD", "LÕPLIK HIND:".bold().green(), kokku_usd);
            }
        }
        println!(
            "{}",
            "  NB! See on börsi (spot) hind. Ostjad maksavad tavaliselt veidi vähem, müüjad küsivad veidi rohkem."
                .italic()
                .dimmed()
        );

        // 6. Uuesti?
        if loe_rida("\nVeel üks arvutus? (j/e): ").to_lowercase() != "j" {
            return;
        }
    }
}

#[cfg(test)]
mod testid {
    use super::*;

    #[test]
    fn uks_unts_puhast_metalli_on_untsi_hind() {
        let v = arvuta_vaartus(2000.0, GRAMMI_UNTSIS, 1000.0);
        assert!((v - 2000.0).abs() < 0.001);
    }

    #[test]
    fn proov_vahendab_vaartust() {
        let puhas = arvuta_vaartus(2000.0, 10.0, 1000.0);
        let osa = arvuta_vaartus(2000.0, 10.0, 750.0);
        assert!((osa - puhas * 0.75).abs() < 0.001);
    }
}
