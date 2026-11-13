//! Andmete laadimine.
//!
//! Hinnad tulevad Yahoo Finance'i avalikust graafikuteenusest (API võtit pole vaja).
//! Hinnad on USD troy untsi kohta (1 troy unts ≈ 31,1 grammi).

use serde::Deserialize;
use std::error::Error;
use std::time::Duration;

/// Üks väärismetall: inimloetav nimi + Yahoo Finance'i sümbol.
pub struct Metall {
    pub nimi: &'static str,
    pub symbol: &'static str,
}

/// Metallid, mida jälgime. Siia saab lihtsalt juurde lisada.
pub const METALLID: [Metall; 4] = [
    Metall { nimi: "Kuld", symbol: "GC=F" },
    Metall { nimi: "Hõbe", symbol: "SI=F" },
    Metall { nimi: "Plaatina", symbol: "PL=F" },
    Metall { nimi: "Pallaadium", symbol: "PA=F" },
];

// ---- Struktuurid, mis vastavad Yahoo vastuse JSON-ile ----
// Meid huvitab ainult sulgemishindade list, seega muud välja jätame.

#[derive(Deserialize)]
struct YahooVastus {
    chart: Chart,
}

#[derive(Deserialize)]
struct Chart {
    result: Option<Vec<Tulemus>>,
}

#[derive(Deserialize)]
struct Tulemus {
    indicators: Indikaatorid,
}

#[derive(Deserialize)]
struct Indikaatorid {
    quote: Vec<Quote>,
}

#[derive(Deserialize)]
struct Quote {
    // Mõnel päeval võib hind puududa, seepärast Option
    close: Vec<Option<f64>>,
}

/// Laeb viimase ~6 kuu päevased sulgemishinnad (vanim hind enne, uusim viimasena).
pub fn lae_hinnad(symbol: &str) -> Result<Vec<f64>, Box<dyn Error>> {
    let url = format!(
        "https://query1.finance.yahoo.com/v8/finance/chart/{}?range=6mo&interval=1d",
        symbol
    );

    // Yahoo nõuab User-Agent päist, muidu võib päringu tagasi lükata
    let klient = reqwest::blocking::Client::builder()
        .user_agent("Mozilla/5.0 (vaarismetallid)")
        .timeout(Duration::from_secs(15))
        .build()?;

    let vastus: YahooVastus = klient.get(&url).send()?.error_for_status()?.json()?;

    let tulemus = vastus
        .chart
        .result
        .and_then(|r| r.into_iter().next())
        .ok_or("Yahoo ei tagastanud andmeid")?;

    let quote = tulemus
        .indicators
        .quote
        .into_iter()
        .next()
        .ok_or("Hinnalist puudub")?;

    // Jätame välja puuduvad päevad (None) ja jääb puhas hindade list
    let hinnad: Vec<f64> = quote.close.into_iter().flatten().collect();

    if hinnad.is_empty() {
        return Err("Hinnalist on tühi".into());
    }
    Ok(hinnad)
}

/// Laeb praeguse EUR/USD kursi, ehk mitu USD-d on 1 euro eest.
/// Hinna eurodesse teisendamiseks: hind_usd / kurss.
pub fn lae_eur_usd_kurss() -> Result<f64, Box<dyn Error>> {
    let hinnad = lae_hinnad("EURUSD=X")?;
    hinnad
        .last()
        .copied()
        .ok_or_else(|| "Kurss puudub".into())
}
