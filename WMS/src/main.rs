mod towar;
mod operacje;
use towar::Towar; 
use std::io::{self, Write};
use std::fs;

fn main() {
    let nazwa_pliku = "baza_towaru.json";

    // Odczyt, próbujemy wczytać plik jeśli go nie ma to tworzymy pusty wektor
    let mut magazyn: Vec<Towar> = match fs::read_to_string(nazwa_pliku) {
        Ok(zawartosc_pliku) => {
            // Udało sie odczytać plik! Deserialuzjemy tekst JSON z powrotem
            serde_json::from_str(&zawartosc_pliku).expect("Błąd: Plik json jest uszkodzony")
        }
        Err(_) => {
            // Plik nie istnieje, zaczynamy z czysta karta
            Vec::new()
        }
    };

    let mut aktualne_id = 1;
    for przedmiot in &magazyn {
        if przedmiot.id >= aktualne_id {
            aktualne_id = przedmiot.id + 1;
        }
    }

    loop {

        println!("=========================================\nMAGAZYN ZWROTÓW KONSUMENCKICH\n=========================================\nWybierz operację:\n[1] - Dodaj nowy towar do magazynu\n[2] - Wyszukaj i wyświetl produkt\n[3] - Usuń towar z magazynu\n[0] - Wyjście \n=========================================");
        print!("Twój wybór: ");
        io::stdout().flush().expect("Błąd");

        let mut wejscie_menu = String::new();
        io::stdin().read_line(&mut wejscie_menu).expect("Błąd odczytu");
        // Parsowanie na liczbe
        let wybor: i32 = wejscie_menu.trim().parse().expect("To nie jest liczba!");

        match wybor {
            1 => {
                    operacje::dodaj_towar(&mut magazyn, &mut aktualne_id);    
                }

            2 => {
                    operacje::wyszukaj_towar(&magazyn);
                }
            3 => {
                   operacje::usun_towar(&mut magazyn); 
                }
                0 => {
                    println!("Zamykanie programu. Do zobaczenia!");
                    break;
                }
                _ => {
                    println!("Nieprawidłowy wybór wpisz 1, 2 lub 0.");
                }
                
            }
        } 
    
    // Zapis do pliku
    // Zamieniamy cały wektor na ładny tekst formatu JSON
    let json_tekst = serde_json::to_string_pretty(&magazyn).expect("Błąd przy serializacji");
    // Fizycznie zapisujemy wygenerowany tekst do pliku na dysku.
    fs::write(nazwa_pliku, json_tekst).expect("Błąd przy zapisywaniu pliu na dysku.");

    println!("Zapisano bazę towarów na dysku");

}

