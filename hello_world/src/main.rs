use std::io;
fn main() {
    let mut waga = String::new();
    println!("Podaj swoją wagę");
    io::stdin().read_line(&mut waga).expect("Błąd odczytu");
    let waga: f64 = waga.trim().parse().expect("To nie jest poprawna liczba!");
    let kalorie: f64 = oblicz_kalorie(waga);
    
    let gramy_bialka: f64 = bialko_gramy(waga);
    let kcal_b: f64 = bialko_kcal(gramy_bialka);

    let tluszcze_gramy: f64 = tluszcze_gramy(waga);
    let tluszcze_kcal: f64 = tluszcze_kcal(tluszcze_gramy);

    println!("Twoje zapotrzebowanie na białko wynosi {}g oraz zapotrzebowanie na kcal wynosi {}kcal", bialko, kalorie);
    
}

fn oblicz_kalorie(waga: f64) -> f64{waga * 24.0}

fn bialko_gramy(waga: f64) -> f64 {waga * 2.0}
fn bialko_kcal(gramy_bialka: f64) -> f64 {gramy_bialka * 4.0}

fn tluszcze_gramy(waga: f64) -> f64{waga * 1.0}
fn tluszcze_kcal(tluszcze_gramy: f64) -> f64 {tluszcze_gramy * 9.0}

fn pula_na_wegle(waga: f64) -> f64{oblicz_kalorie(waga) - 
