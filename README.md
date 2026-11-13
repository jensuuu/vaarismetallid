# Väärismetallide hinnad ja soovitused (Rust)

Terminaliprogramm, mis näitab kulla, hõbeda, plaatina ja pallaadiumi hindu (USD ja EUR), annab lihtsa osta/müü soovituse ning arvutab kaalu järgi lõpliku hinna.


Windowsi .exe ehitamiseks Linuxis on lisaks vaja: `sudo dnf install mingw64-gcc`

Make'i ei pea kasutama, skriptid töötavad ka otse: `bash scripts/ehita_linux.sh`


## Menüü

1. **Hinnad ja soovitused** – kõik metallid korraga
2. **Hinnakalkulaator** – vali metall, ühik (g, kg, troy unts), kaal ja proov (nt 999, 925, 750, 585). Programm arvutab puhta metalli kaalu ja lõpliku hinna USD-des ja EUR-ides.

## Failid

| Fail | Ülesanne |
|------|----------|
| `Makefile` | Lühikesed käsud (`make linux`, `make windows`, ...) |
| `scripts/ehita_linux.sh` | Ehitab Linuxi versiooni |
| `scripts/ehita_windows.sh` | Ehitab Windowsi .exe Linuxis |
| `scripts/ehita_windows.bat` | Ehitab programmi Windowsis |
| `scripts/paketi.sh` | Ehitab ja pakib mõlemad versioonid |
| `scripts/paigalda_linux.sh` | Paigaldab programmi ~/.local/bin kausta |
| `scripts/yhine.sh` | Skriptide ühised abifunktsioonid |
| `src/main.rs` | Programmi algus ja menüü |
| `src/andmed.rs` | Hindade laadimine (Yahoo Finance, API võtit pole vaja) |
| `src/analuus.rs` | Näitajate arvutus ja soovituse loogika |
| `src/kuvamine.rs` | Värviline väljund terminalis |
| `src/kalkulaator.rs` | Hinnakalkulaator |

## Kuidas soovitus tekib

Iga näitaja annab punkte (positiivne = osta, negatiivne = müü):

- **RSI (14 päeva):** alla 30 → +2, alla 40 → +1, üle 60 → -1, üle 70 → -2
- **Hind vs 50 päeva keskmine:** üle 5% alla → +1, üle 8% üle → -1
- **Asukoht poole aasta vahemikus:** alumised 20% → +1, ülemised 10% → -1

Kokku: ≥3 tugev ost, 1–2 ost, 0 oota, -1…-2 müü, ≤-3 tugev müük.

## Uue metalli lisamine

Lisa rida `src/andmed.rs` failis olevasse `METALLID` listi (ja muuda listi pikkust).

## Hoiatus

See on õppeprojekt, mitte finantsnõu. Tehnilised näitajad ei ennusta tulevikku. Yahoo Finance'i andmed on mitteametlik allikas ja võivad muutuda või kaduda.
