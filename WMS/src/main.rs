use std::io::{self, Write};

struct Towar {
    nazwa: String,
    marka: String,
    rozmiar: i32,
    stan: String,
    id: i32,
}

fn main() {
    let mut magazyn: Vec<Towar> = Vec::new();
    let mut aktualne_id = 1;

    loop {

        println!("=========================================\nMAGAZYN ZWROTÓW KONSUMENCKICH\n=========================================\\\nWybierz operację:\n[1] - Dodaj nowy towar z palety\n[2] - Wyszukaj i wyświetl produkt\n[0] - Wyjście / Zamknij program\n=========================================");
        print!("Twój wybór: ");
        io::stdout().flush().expect("Błąd");

        let mut wejscie_menu = String::new();
        io::stdin().read_line(&mut wejscie_menu).expect("Błąd odczytu");
        // Parsowanie na liczbe
        let wybor: i32 = wejscie_menu.trim.parse.expect("To nie jest liczba!");

        match wybor {
            1 => {
                    // Pobieramy dane
                    print!("Podaj nazwe przedmiotu: ");
                    // Wypchniecie tekstu na ekran
                    io::stdout().flush().expect("Błąd przy wyświetlaniu tekstu");
                    let mut wejsciowa_nazwa = String::new();
                    io::stdin().read_line(&mut wejsciowa_nazwa).expect("Błąd odczytu");
                    // Czyścimy z białych znaków (np. enter)
                    let nazwa_gotowa = wejsciowa_nazwa.trim().to_string();

                    print!("Podaj markę przedmiotu: ");
                    io::stdout().flush().expect("Błąd przy wyświetlaniu tekstu");
                    let mut wejsciowa_marka = String::new();
                    io::stdin().read_line(&mut wejsciowa_marka).expect("Błąd odczytu");
                    let marka_gotowa = wejsciowa_marka.trim().to_string();

                    print!("Podaj rozmiar przedmiotu: ");
                    io::stdout().flush().expect("Błąd przy wyświetlaniu tekstu");
                    let mut wejsciowy_rozmiar = String::new();
                    io::stdin().read_line(&mut wejsciowy_rozmiar).expect("Błąd odczytu");
                    let rozmiar_gotowy = wejsciowy_rozmiar.trim().parse().expect("To nie jest poprawna liczba!");

                    print!("Podaj stan przedmiotu (A, B, C, D): ");
                    io::stdout().flush().expect("Błąd przy wyświetlaniu tekstu");
                    let mut wejsciowy_stan = String::new();
                    io::stdin().read_line(&mut wejsciowy_stan).expect("Błąd odczytu");
                    let stan_gotowy = wejsciowy_stan.trim().to_string();

                    let nowy_towar = Towar {
                        nazwa: nazwa_gotowa,
                        marka: marka_gotowa,
                        rozmiar: rozmiar_gotowy,
                        stan: stan_gotowy,
                        id: aktualne_id,
                    };

                    magazyn.push(nowy_towar);
                    aktualne_id += 1;

                    println!("Przedmiot został dodany! W magazynie jest teraz {} przedmiotów", magazyn.len());
                }

                2 => {


                }
                0 => {
                    println!("Zamykanie programu. Do zobaczenia!");
                    break;
                }
                _ => {
                    println!("Nieprawidłowy wybór wpisz 1, 2 lub 0.";)
                }
                
            }
        } 








        

}
