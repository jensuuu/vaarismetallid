//! Hinnaanalüüs ja soovitused.
//!
//! Loogika on tahtlikult lihtne. Iga näitaja annab punkte:
//!   positiivne = pigem OSTA, negatiivne = pigem MÜÜ.
//! Punktid liidetakse kokku ja kokkuvõttest tuleb soovitus.

/// Lõplik soovitus.
#[derive(Debug, PartialEq)]
pub enum Soovitus {
    TugevOst,
    Ost,
    Oota,
    Muuk,
    TugevMuuk,
}

/// Kõik arvutatud näitajad ühe metalli kohta.
pub struct Analuus {
    pub hind: f64,
    pub muutus_1p: Option<f64>,  // % muutus 1 päevaga
    pub muutus_30p: Option<f64>, // % muutus 30 päevaga
    pub sma20: Option<f64>,      // 20 päeva libisev keskmine
    pub sma50: Option<f64>,      // 50 päeva libisev keskmine
    pub rsi: Option<f64>,        // RSI (0..100)
    pub vahemiku_asukoht: f64,   // 0.0 = perioodi madalaim, 1.0 = perioodi kõrgeim
    pub punktid: i32,
    pub pohjendused: Vec<String>,
    pub soovitus: Soovitus,
}

/// Libisev keskmine viimasest `n` hinnast.
fn keskmine(hinnad: &[f64], n: usize) -> Option<f64> {
    if hinnad.len() < n {
        return None;
    }
    let viimased = &hinnad[hinnad.len() - n..];
    Some(viimased.iter().sum::<f64>() / n as f64)
}

/// Protsentuaalne muutus `paeva` päeva taguse hinnaga võrreldes.
fn muutus_protsent(hinnad: &[f64], paeva: usize) -> Option<f64> {
    if hinnad.len() <= paeva {
        return None;
    }
    let praegu = hinnad[hinnad.len() - 1];
    let varem = hinnad[hinnad.len() - 1 - paeva];
    Some((praegu / varem - 1.0) * 100.0)
}

/// RSI (Relative Strength Index) lihtsustatud kujul.
/// Alla 30 = on tugevasti langenud ("ülemüüdud"), üle 70 = on tugevasti tõusnud ("ülehinnatud").
fn rsi(hinnad: &[f64], periood: usize) -> Option<f64> {
    if hinnad.len() <= periood {
        return None;
    }
    let viimased = &hinnad[hinnad.len() - periood - 1..];

    let mut tulu = 0.0; // päevade tõusude summa
    let mut kaotus = 0.0; // päevade langemiste summa

    for paar in viimased.windows(2) {
        let vahe = paar[1] - paar[0];
        if vahe > 0.0 {
            tulu += vahe;
        } else {
            kaotus += -vahe;
        }
    }

    if kaotus == 0.0 {
        return Some(100.0);
    }
    let suhe = tulu / kaotus;
    Some(100.0 - 100.0 / (1.0 + suhe))
}

/// Peamine funktsioon: võtab hinnad ja annab tagasi analüüsi koos soovitusega.
pub fn analuusi(hinnad: &[f64]) -> Option<Analuus> {
    let hind = *hinnad.last()?;

    let sma20 = keskmine(hinnad, 20);
    let sma50 = keskmine(hinnad, 50);
    let rsi_vaartus = rsi(hinnad, 14);

    // Kus hind asub perioodi madalaima ja kõrgeima vahel (0..1)
    let madalaim = hinnad.iter().cloned().fold(f64::INFINITY, f64::min);
    let korgeim = hinnad.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let vahemiku_asukoht = if korgeim > madalaim {
        (hind - madalaim) / (korgeim - madalaim)
    } else {
        0.5
    };

    let mut punktid = 0;
    let mut pohjendused: Vec<String> = Vec::new();

    // --- 1. RSI ---
    if let Some(r) = rsi_vaartus {
        if r < 30.0 {
            punktid += 2;
            pohjendused.push(format!("RSI {:.0} on väga madal – hind on tugevasti langenud", r));
        } else if r < 40.0 {
            punktid += 1;
            pohjendused.push(format!("RSI {:.0} on madalavõitu", r));
        } else if r > 70.0 {
            punktid -= 2;
            pohjendused.push(format!("RSI {:.0} on väga kõrge – hind on kiiresti tõusnud", r));
        } else if r > 60.0 {
            punktid -= 1;
            pohjendused.push(format!("RSI {:.0} on kõrgevõitu", r));
        }
    }

    // --- 2. Hind võrreldes 50 päeva keskmisega ---
    if let Some(s50) = sma50 {
        let kõrvalekalle = (hind / s50 - 1.0) * 100.0;
        if kõrvalekalle < -5.0 {
            punktid += 1;
            pohjendused.push(format!(
                "hind on {:.1}% alla 50 päeva keskmise (võib olla soodne)",
                -kõrvalekalle
            ));
        } else if kõrvalekalle > 8.0 {
            punktid -= 1;
            pohjendused.push(format!(
                "hind on {:.1}% üle 50 päeva keskmise (võib olla kallis)",
                kõrvalekalle
            ));
        }
    }

    // --- 3. Asukoht poole aasta vahemikus ---
    if vahemiku_asukoht < 0.2 {
        punktid += 1;
        pohjendused.push("hind on poole aasta madalaimate hulgas".to_string());
    } else if vahemiku_asukoht > 0.9 {
        punktid -= 1;
        pohjendused.push("hind on poole aasta kõrgeimate hulgas".to_string());
    }

    // --- Punktidest soovituseks ---
    let soovitus = if punktid >= 3 {
        Soovitus::TugevOst
    } else if punktid >= 1 {
        Soovitus::Ost
    } else if punktid == 0 {
        Soovitus::Oota
    } else if punktid > -3 {
        Soovitus::Muuk
    } else {
        Soovitus::TugevMuuk
    };

    if pohjendused.is_empty() {
        pohjendused.push("ükski näitaja ei anna selget signaali".to_string());
    }

    Some(Analuus {
        hind,
        muutus_1p: muutus_protsent(hinnad, 1),
        muutus_30p: muutus_protsent(hinnad, 30),
        sma20,
        sma50,
        rsi: rsi_vaartus,
        vahemiku_asukoht,
        punktid,
        pohjendused,
        soovitus,
    })
}
