use crate::towar::{StanTowaru, Towar};
use std::io::{self, Write};

pub fn dodaj_towar(magazyn: &mut Vec<Towar>, aktualne_id: &mut i32) {
    // Pobieramy dane
    
    print!("Podaj markę przedmiotu: ");
    io::stdout().flush().expect("Błąd przy wyświetlaniu tekstu");
    let mut wejsciowa_marka = String::new();
    io::stdin()
        .read_line(&mut wejsciowa_marka)
        .expect("Błąd odczytu");
    let marka_gotowa = wejsciowa_marka.trim().to_string();

    print!("Podaj kolor przedmiotu: ");
    // Wypchniecie tekstu na ekran
    io::stdout().flush().expect("Błąd przy wyświetlaniu tekstu");
    let mut wejsciowy_kolor = String::new();
    io::stdin()
        .read_line(&mut wejsciowy_kolor)
        .expect("Błąd odczytu");
    // Czyścimy z białych znaków (np. enter)
    let kolor_gotowa = wejsciowy_kolor.trim().to_string();


    print!("Podaj rozmiar przedmiotu: ");
    io::stdout().flush().expect("Błąd przy wyświetlaniu tekstu");
    let mut wejsciowy_rozmiar = String::new();
    io::stdin()
        .read_line(&mut wejsciowy_rozmiar)
        .expect("Błąd odczytu");
    let rozmiar_gotowy = match wejsciowy_rozmiar.trim().parse() {
        Ok(liczba) => liczba,
        Err(_) => {
            println!("Błąd: Podany rozmiar nie jest poprawną liczbą! Przerywam dodawanie towaru.");
            return;
        }
    };

    print!("Podaj stan przedmiotu (A, B, C, D): ");
    io::stdout().flush().expect("Błąd przy wyświetlaniu tekstu");

    // Najpierw tworzymy zmienną i pobieramy tekst!
    let mut wejsciowy_stan = String::new();
    io::stdin()
        .read_line(&mut wejsciowy_stan)
        .expect("Błąd odczytu");

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
    
    // Pobieranie ceny zakupu
    print!("Podaj cenę zakupu (np. 15.50): ");
    io::stdout().flush().expect("Błąd przy wyświetlaniu tekstu");
    let mut wejsciowa_cena_z = String::new();
    io::stdin().read_line(&mut wejsciowa_cena_z).expect("Błąd odczytu");
    let cena_zakupu_gotowa: f32 = match wejsciowa_cena_z.trim().parse() {
        Ok(liczba) => liczba,
        Err(_) => {
            println!("Błąd: Podana cena nie jest poprawną liczbą! Przerywam dodawanie.");
            return;
        }
    };

    // Pobieranie ceny sprzedaży
    print!("Podaj przewidwyana cene sprzedaży (np. 49.99): ");
    io::stdout().flush().expect("Błąd przy wyświetlaniu tekstu");
    let mut wejsciowa_cena_s = String::new();
    io::stdin().read_line(&mut wejsciowa_cena_s).expect("Błąd odczytu");
    let cena_sprzedazy_gotowa: f32 = match wejsciowa_cena_s.trim().parse() {
        Ok(liczba) => liczba,
        Err(_) => {
            println!("Błąd: Podana cena nie jest poprawną liczbą! Przerywam dodawanie.");
            return;
        }
    };


    let nowy_towar = Towar {
        nazwa: kolor_gotowa,
        marka: marka_gotowa,
        rozmiar: rozmiar_gotowy,
        stan: stan_gotowy,
        cena_zakupu: cena_zakupu_gotowa,
        cena_sprzedazy: cena_sprzedazy_gotowa,
        id: *aktualne_id,
    };

    magazyn.push(nowy_towar);
    *aktualne_id += 1;

    println!(
        "Przedmiot został dodany! W magazynie jest teraz {} przedmiotów",
        magazyn.len()
    );
}

pub fn wyszukaj_towar(magazyn: &Vec<Towar>) {
    loop {
        print!(
            " [1] - Szukaj po marce\n [2] - Szukaj po rozmiarze\n [3] - Szukaj po stanie\n [0] - Powrót do głównego menu\n"
        );
        print!("Twój wybór: ");
        io::stdout().flush().expect("Błąd");

        let mut wejscie_podmenu = String::new();
        io::stdin()
            .read_line(&mut wejscie_podmenu)
            .expect("Błąd odczytu");
        // Parsowanie na liczbe
        let wybor2: i32 = match wejscie_podmenu.trim().parse() {
            Ok(liczba) => liczba,
            Err(_) => {
                println!("Błąd: To nie jest poprawna liczba! Spróbuj ponownie.");
                continue;
            }
        };

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
                for przedmiot in magazyn {
                    // Porównujemy tekst z wektora z tekstem wpisanym przez Ciebie
                    if przedmiot.marka == szukana_marka {
                        println!(
                            "ID: [{}] | {} | Rozmiar: {} | Stan: {:?}",
                            przedmiot.id, przedmiot.marka, przedmiot.rozmiar, przedmiot.stan
                        );

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
                let szukany_rozmiar: i32 = match wpisany_rozmiar.trim().parse() {
                    Ok(liczba) => liczba,
                    Err(_) => {
                        println!("Błąd: To nie jest poprawny rozmiar! Przerywam operację.");
                        continue;
                    }
                };

                let mut znaleziono = false;
                println!("--- WYNIKI WYSZUKIWANIA ---");

                for przedmiot in magazyn {
                    if przedmiot.rozmiar == szukany_rozmiar {
                        println!(
                            "ID: [{}] | {} | Rozmiar: {} | Stan: {:?}",
                            przedmiot.id, przedmiot.marka, przedmiot.rozmiar, przedmiot.stan
                        );

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
                for przedmiot in magazyn {
                    // Porównujemy tekst z wektora z tekstem wpisanym przez Ciebie
                    if przedmiot.stan == szukany_stan_enum {
                        println!(
                            "ID: [{}] | {} | Rozmiar: {} | Stan: {:?}",
                            przedmiot.id, przedmiot.marka, przedmiot.rozmiar, przedmiot.stan
                        );

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

pub fn usun_towar(magazyn: &mut Vec<Towar>) {
    if magazyn.is_empty() {
        println!("Magazyn jest obecnie pusty");
    } else {
        println!("--- Lista Towarów ---");
        for przedmiot in magazyn.iter() {
            println!(
                "ID [{}] | {} (Rozmiar: {}, Stan: {:?}, Kolor: {})",
                przedmiot.id, przedmiot.marka, przedmiot.rozmiar, przedmiot.stan, przedmiot.nazwa
            );
        }
        print!("Podaj ID które chcesz usunąć: ");
        io::stdout().flush().expect("Błąd");
        let mut wpisane_id = String::new();
        io::stdin().read_line(&mut wpisane_id).expect("Błąd");
        // Zamieniamy na i32
        let gotowe_id: i32 = match wpisane_id.trim().parse() {
            Ok(liczba) => liczba,
            Err(_) => {
                println!("Błąd: To nie jest poprawne ID! Przerywam operację");
                return;
            }
        };

        // Szukamy pozycji przedmiotu o podanym id
        let pozycja = magazyn.iter().position(|p| p.id == gotowe_id);

        // Sprawdzamy wynik i usuwamy
        match pozycja {
            Some(indeks) => {
                let sprzedany_towar = magazyn.remove(indeks);
                let zysk = sprzedany_towar.cena_sprzedazy -sprzedany_towar.cena_zakupu;
                println!("=========================================");
                println!("Sukces: Towar ID {}, {} został wydany z magazynu", sprzedany_towar.id, sprzedany_towar.marka);
                println!("Kupiono za: {:.2}zł | Sprzedano za: {:.2}zł", sprzedany_towar.cena_zakupu, sprzedany_towar.cena_sprzedazy);
                println!("Wygenerowano zysk: {:.2}zł", zysk);
                println!("=========================================");
            }
            None => {
                println!("Błąd: Nie mamy w magazynie towaru o ID {}.", gotowe_id);
            }
        }
    }
}
