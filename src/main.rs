//! Väärismetallide hinnad, soovitused ja hinnakalkulaator.
//!
//! Menüü:
//!   1) hinnad ja osta/müü soovitused  (moodulid `andmed`, `analuus`, `kuvamine`)
//!   2) hinnakalkulaator: sisestad kaalu, saad lõpliku hinna  (moodul `kalkulaator`)

mod analuus;
mod andmed;
mod kalkulaator;
mod kuvamine;

fn main() {
    // Windowsis lülitame sisse värvikoodide toe (Windows 10 ja uuem)
    #[cfg(windows)]
    let _ = colored::control::set_virtual_terminal(true);

    loop {
        println!();
        println!("=== VÄÄRISMETALLID ===");
        println!("  1) Hinnad ja soovitused");
        println!("  2) Hinnakalkulaator (kaal -> hind)");
        println!("  0) Välju");

        match kalkulaator::loe_rida("Vali: ").as_str() {
            "1" => naita_hinnad(),
            "2" => kalkulaator::kaivita(),
            "0" => break,
            _ => println!("Vigane valik."),
        }
    }

    // Windowsis aken sulgub kohe, kui .exe-d topeltklikiga käivitada,
    // seega ootame enne sulgemist Enterit
    #[cfg(windows)]
    {
        println!("\nVajuta Enter, et sulgeda...");
        kalkulaator::loe_rida("");
    }
}

/// Näitab kõigi metallide hindu ja soovitusi.
fn naita_hinnad() {
    // Kui kursi laadimine ebaõnnestub, näitame hindu ainult USD-des
    let kurss = match andmed::lae_eur_usd_kurss() {
        Ok(k) => Some(k),
        Err(viga) => {
            eprintln!("EUR kursi laadimine ebaõnnestus ({}), näitan ainult USD\n", viga);
            None
        }
    };

    kuvamine::prindi_pais(kurss);

    for metall in andmed::METALLID.iter() {
        let hinnad = match andmed::lae_hinnad(metall.symbol) {
            Ok(h) => h,
            Err(viga) => {
                eprintln!("{}: hindade laadimine ebaõnnestus ({})\n", metall.nimi, viga);
                continue;
            }
        };

        match analuus::analuusi(&hinnad) {
            Some(tulemus) => kuvamine::prindi_metall(metall.nimi, &tulemus, kurss),
            None => eprintln!("{}: analüüsiks pole piisavalt andmeid\n", metall.nimi),
        }
    }

    kuvamine::prindi_hoiatus();
}
