mod towar; 

use std::io::{self, Write};
use std::fs;

use towar::{Towar, StanTowaru};

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

                    // Najpierw tworzymy zmienną i pobieramy tekst!
                    let mut wejsciowy_stan = String::new();
                    io::stdin().read_line(&mut wejsciowy_stan).expect("Błąd odczytu");

                    // A dopiero potem czyścimy i robimy z tego Enum
                    let wpisany_tekst = wejsciowy_stan.trim().to_uppercase();
                    let stan_gotowy = match wpisany_tekst.as_str() {
                        "A" => StanTowaru::A,
                        "B" => StanTowaru::B,
                        "C" => StanTowaru::C,
                        "D" => StanTowaru::D,
                        _ => {
                            println!("Błędny stan! Domyślnie ustawiam stan na C.");
                            StanTowaru::C
                            }                        
                    };

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
                    loop {
                        print!(" [1] - Szukaj po marce\n [2] - Szukaj po rozmiarze\n [3] - Szukaj po stanie\n [0] - Powrót do głównego menu\n");
                        print!("Twój wybór: ");
                        io::stdout().flush().expect("Błąd");

                        let mut wejscie_podmenu = String::new();
                        io::stdin().read_line(&mut wejscie_podmenu).expect("Błąd odczytu");
                        // Parsowanie na liczbe
                        let wybor2: i32 = wejscie_podmenu.trim().parse().expect("To nie jest liczba!");

                        match wybor2 {
                            1 => {
                                // Pobieramy dane od użytkownika i czyścimy je
                                print!("Podaj marke której szukasz: ");
                                io::stdout().flush().expect("Błąd");
                                let mut wpisana_marka = String::new();
                                io::stdin().read_line(&mut wpisana_marka).expect("Błąd");
                                let szukana_marka = wpisana_marka.trim().to_string();
                                
                                // Flaga Pomocnicza
                                let mut znaleziono = false;
                                println!("--- WYNIKI WYSZUKIWANIA ---");

                                // Pętla przechodzaca przez magazyn
                                for przedmiot in &magazyn {
                                    // Porównujemy tekst z wektora z tekstem wpisanym przez Ciebie
                                    if przedmiot.marka == szukana_marka {
                                        println!("ID: [{}] | {} | Rozmiar: {} | Stan: {:?}", przedmiot.id, przedmiot.marka, przedmiot.rozmiar, przedmiot.stan);

                                        znaleziono = true;
                                    }
                                }
                                // Komunikat jeżeli nic nie znaleziono
                                if znaleziono == false {
                                    println!("Nie znaleziono w magazynie towaru tej marki");
                                }
                            }
                            2 => {
                                // Pobieramy dane od użytkownika i czyścimy je
                                print!("Podaj rozmiar który szukasz: ");
                                io::stdout().flush().expect("Błąd");
                                let mut wpisany_rozmiar = String::new();
                                io::stdin().read_line(&mut wpisany_rozmiar).expect("Błąd");
                                // Zamieniamy na i32
                                let szukany_rozmiar: i32 = wpisany_rozmiar.trim().parse().expect("To nie jest liczba!");

                                let mut znaleziono = false;
                                println!("--- WYNIKI WYSZUKIWANIA ---");

                                for przedmiot in &magazyn {
                                    if przedmiot.rozmiar == szukany_rozmiar {
                                        println!("ID: [{}] | {} | Rozmiar: {} | Stan: {:?}", przedmiot.id, przedmiot.marka, przedmiot.rozmiar, przedmiot.stan);

                                        znaleziono = true;
                                    }
                                }
                                if znaleziono == false {
                                    println!("Nie znaleziono w magazynie towaru w tym rozmiarze");
                                }
                            }
                                 3 => {
                                print!("Podaj stan który szukasz: ");
                                io::stdout().flush().expect("Błąd");
                                let mut wpisany_stan = String::new();
                                io::stdin().read_line(&mut wpisany_stan).expect("Błąd");

                                // Tłumaczymy to na dużą literę
                                let szukany_tekst = wpisany_stan.trim().to_uppercase();

                                // Tworzymy Enum do wyszukiwania
                                let szukany_stan_enum = match szukany_tekst.as_str() {
                                    "A" => StanTowaru::A,
                                    "B" => StanTowaru::B,
                                    "C" => StanTowaru::C,
                                    "D" => StanTowaru::D,
                                    _ => {
                                        println!("Nie ma takiego stanu! Szukam domyślnie A.");
                                        StanTowaru::A
                                        }
                                    };
                                // Flaga Pomocnicza
                                let mut znaleziono = false;
                                println!("--- WYNIKI WYSZUKIWANIA ---");

                                // Pętla przechodzaca przez magazyn
                                for przedmiot in &magazyn {
                                    // Porównujemy tekst z wektora z tekstem wpisanym przez Ciebie
                                    if przedmiot.stan == szukany_stan_enum {
                                        println!("ID: [{}] | {} | Rozmiar: {} | Stan: {:?}", przedmiot.id, przedmiot.marka, przedmiot.rozmiar, przedmiot.stan);

                                        znaleziono = true;
                                    }
                                }
                                // Komunikat jeżeli nic nie znaleziono
                                if znaleziono == false {
                                    println!("Nie znaleziono w magazynie towaru w tym stanie.");
                                }
                            }

                            0 => {
                                println!("Powrót do głównego menu!");
                                break;
                            }
                            _ => {
                                println!("Nieprawidłowy wybór wpisz 1, 2, 3 lub 0");
                            }
                        }
                    }


                }
                3 => {
                    if magazyn.is_empty() {
                        println!("Magazyn jest obecnie pusty");
                    } else {
                        println!("--- Lista Towarów ---");
                        for przedmiot in &magazyn {
                            println!("ID [{}] | {} (Rozmiar: {}, Stan: {:?})", przedmiot.id, przedmiot.nazwa, przedmiot.rozmiar, przedmiot.stan);
                        }
                        print!("Podaj ID które chcesz usunąć: ");
                                io::stdout().flush().expect("Błąd");
                                let mut wpisane_id = String::new();
                                io::stdin().read_line(&mut wpisane_id).expect("Błąd");
                                // Zamieniamy na i32
                                let gotowe_id: i32 = wpisane_id.trim().parse().expect("To nie jest liczba!");

                                // Szukamy pozycji przedmiotu o podanym id
                                let pozycja = magazyn.iter().position(|p| p.id == gotowe_id);

                                // Sprawdzamy wynik i usuwamy
                                match pozycja {
                                    Some(indeks ) => {
                                        magazyn.remove(indeks);
                                        println!("Sukces: Towar od ID {} został usunięty.", gotowe_id);                                        
                                    } 
                                    None => {
                                        println!("Błąd: Nie mamy w magazynie towaru o ID {}.", gotowe_id);
                                    }
                                    
                                }

                    }
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
