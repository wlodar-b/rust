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

        println!("=========================================\nMAGAZYN ZWROTÓW KONSUMENCKICH\n=========================================\nWybierz operację:\n[1] - Dodaj nowy towar do magazynu\n[2] - Wyszukaj i wyświetl produkt\n[0] - Wyjście \n=========================================");
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
                                        println!("ID: [{}] | {} | Rozmiar: {} | Stan: {}", przedmiot.id, przedmiot.marka, przedmiot.rozmiar, przedmiot.stan);

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
                                        println!("ID: [{}] | {} | Rozmiar: {} | Stan: {}", przedmiot.id, przedmiot.marka, przedmiot.rozmiar, przedmiot.stan);

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
                                let szukany_stan = wpisany_stan.trim().to_string();
                                
                                // Flaga Pomocnicza
                                let mut znaleziono = false;
                                println!("--- WYNIKI WYSZUKIWANIA ---");

                                // Pętla przechodzaca przez magazyn
                                for przedmiot in &magazyn {
                                    // Porównujemy tekst z wektora z tekstem wpisanym przez Ciebie
                                    if przedmiot.stan == szukany_stan {
                                        println!("ID: [{}] | {} | Rozmiar: {} | Stan: {}", przedmiot.id, przedmiot.marka, przedmiot.rozmiar, przedmiot.stan);

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
                0 => {
                    println!("Zamykanie programu. Do zobaczenia!");
                    break;
                }
                _ => {
                    println!("Nieprawidłowy wybór wpisz 1, 2 lub 0.");
                }
                
            }
        } 
}
