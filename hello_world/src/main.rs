use std::io;
fn main() {
    let mut waga = String::new();
    println!("Podaj swoją wagę w kg");
    io::stdin().read_line(&mut waga).expect("Błąd odczytu");
    let waga: f64 = waga.trim().parse().expect("To nie jest poprawna liczba!");

    let mut wzrost = String::new();
    println!("Podaj swój wzrost w cm");
    io::stdin().read_line(&mut wzrost).expect("Błąd odczytu");
    let wzrost: f64 = wzrost.trim().parse().expect("To nie jest poprawna liczba!");

    let mut wiek  = String::new();
    println!("Podaj swój wiek");
    io::stdin().read_line(&mut wiek).expect("Błąd odczytu");
    let wiek: f64 = wiek.trim().parse().expect("To nie jest poprawna liczba!");

    println!("Podaj płeć\n1 => Mężczyzna\n2 => Kobieta");

    let mut wejscie_plec = String::new();
    io::stdin().read_line(&mut wejscie_plec).expect("Błąd odczytu");
    let wybor_cyfry = wejscie_plec.trim().parse().expect("To nie jest poprawna liczba");

    let modyfikator = match wybor_cyfry {
        1 => 5.0,
        2 => -161.0,
        _ => {
            println!("Nieznany wybór, ustawiam 0.0");
            0.0 // zwraca wartość bez średnika
        }
    };

    println!("Podaj poziom swojej aktywnosci fizycznej:\n 1) Brak aktywności fizycznej, praca za biurkiem.\n 2) Lekka aktywność (lekkie treningi 1-3 razy w tygodniu). \n 3) Średnia aktywność (treningi o umairkowanej intensywności 3-5 razy w tygodniu.\n 4) Wysoka aktywność (ciężkie treningi 6-7 razy w tygodniu).\n 5) Bardzo wysoka aktywność (ciężka praca fizyczna na co dzień + ciężkie treningi.");

    let mut wejscie_aktywnosc = String::new();
    io::stdin().read_line(&mut wejscie_aktywnosc).expect("Błąd odczytu");
    let wybor_aktywnosci: i32 = wejscie_aktywnosc.trim().parse().expect("To nie jest poprawna liczba");

    let aktywnosc: f64 = match wybor_aktywnosci {
        1 => 1.2,
        2 => 1.375,
        3 => 1.55,
        4 => 1.725,
        5 => 1.9,
        _ => {
             println!("Nieznany wybór, ustawiam 0.0");
            0.0 // zwraca wartość bez średnika
        }
    };    

    

    let kalorie: f64 = oblicz_kalorie(waga, wzrost, wiek, modyfikator, aktywnosc);
    
    let gramy_bialka: f64 = bialko_gramy(waga);
    let kcal_b: f64 = bialko_kcal(gramy_bialka);

    let gramy_tluszcze: f64 = tluszcze_gramy(waga);
    let kcal_t: f64 = tluszcze_kcal(gramy_tluszcze);

    let kalorie_na_wegle: f64 = pula_na_wegle(kalorie, kcal_b, kcal_t);
    let gramy_wegle: f64 = wegle_gramy(kalorie_na_wegle);

    println!("Twoje zapotrzebowanie kaloryczne wynosi {}kcal", kalorie);
    println!("Twoje dziennie zapotrzebowanie na makroskładniki wynosi: \nBiałko: {}g\nTłuszcze: {}g\nWęgle: {}g", gramy_bialka, gramy_tluszcze, gramy_wegle);
    
}

fn oblicz_kalorie(waga: f64, wzrost: f64, wiek: f64, modyfikator: f64, aktywnosc: f64) -> f64{((10.0 * waga) + (6.25 * wzrost) - (5.0 * wiek) + modyfikator) * aktywnosc}

fn bialko_gramy(waga: f64) -> f64{waga * 2.0}
fn bialko_kcal(gramy_bialka: f64) -> f64{gramy_bialka * 4.0}

fn tluszcze_gramy(waga: f64) -> f64{waga * 1.0}
fn tluszcze_kcal(gramy_tluszcze: f64) -> f64{gramy_tluszcze * 9.0}

fn pula_na_wegle(kalorie: f64, kcal_b: f64, kcal_t: f64 ) -> f64{kalorie - (kcal_b + kcal_t)}
fn wegle_gramy(kalorie_na_wegle: f64) -> f64{kalorie_na_wegle / 4.0}
