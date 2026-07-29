mod towar;
use serde::Deserialize;
use axum::{routing::{get, post, delete}, Router, extract::{State, Path}, Json};
use towar::Towar;
use std::sync::{Arc, Mutex};

struct AppState { 
    magazyn: Vec<Towar>,
    aktualne_id: i32,
}

#[derive(Deserialize)]
struct NowyTowar {
    nazwa: String,
    marka: String,
    kolor: String,
    rozmiar: i32,
    stan: towar::StanTowaru,
    cena_zakupu: f32,
    cena_sprzedazy: f32,
}

#[tokio::main]
async fn main() {
    // Tworzenie stanu początkowego, owinietego w zamek (Mutex)
    let stan_aplikacji = Arc::new(Mutex::new(AppState {
        magazyn: Vec::new(), // na poczatku magazyn jest pusty
        aktualne_id: 1,
    }));

    // Tworzenie głównego routera aplikacji
    let app = Router::new()
        .route("/", get(|| async { "Witaj w WMS 2.0 (REST API)!"}))
        .route("/magazyn", get(pobierz_magazyn))
        .route("/dodaj", post(dodaj_towar))
        .route("/usun/{id}", delete(usun_towar))
        .route("/magazyn/{id}", get(pobierz_towar))
        .with_state(stan_aplikacji); // podpinamy stan aplikacji do routera

    // Konfigurujemy port na którym serwer będzie nasłuchiwał
    let port = 3000;
    println!("Serwer WMS 2.0 wystartował na porcie: {}", port);

    // Uruchamiamy właściwy nasłuch sieciowy
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port)).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn pobierz_magazyn(
    State(stan): State<Arc<Mutex<AppState>>>,
) -> Json<Vec<Towar>> {
    let sejf = stan.lock().unwrap();
    let kopia_magazynu = sejf.magazyn.clone();
    Json(kopia_magazynu)
}

async fn dodaj_towar(
    State(stan): State<Arc<Mutex<AppState>>>, // State WRAZ z Json działa najlepiej na 1. miejscu
    Json(dane_z_sieci): Json<NowyTowar>,
) -> String {
    let mut sejf = stan.lock().unwrap();
    let nowy_towar = Towar { 
        nazwa: dane_z_sieci.nazwa,
        marka: dane_z_sieci.marka,
        kolor: dane_z_sieci.kolor,
        rozmiar: dane_z_sieci.rozmiar,
        stan: dane_z_sieci.stan,
        id: sejf.aktualne_id,
        cena_zakupu: dane_z_sieci.cena_zakupu,
        cena_sprzedazy: dane_z_sieci.cena_sprzedazy,
    };
    sejf.magazyn.push(nowy_towar);
    sejf.aktualne_id += 1; 
    format!("Sukces! Dodano towar o ID: {}", sejf.aktualne_id - 1)
}

async fn usun_towar(
    State(stan): State<Arc<Mutex<AppState>>>,
    Path(id_do_usuniecia): Path<i32>,
) -> String {
    let mut sejf = stan.lock().unwrap();
    let l_przed = sejf.magazyn.len();
    sejf.magazyn.retain(|towar| towar.id != id_do_usuniecia);
    if l_przed > sejf.magazyn.len() {
        format!("Sukces! Przedmiot o id:{} został usunięty", id_do_usuniecia)
    } else {
        format!("Towar o takim id:{} nie istnieje", id_do_usuniecia)
    }
}

// Zamiast Option<Json<Towar>> zwracamy Result, co w Axumie 0.8+
// rozwiązuje problem z traitem Handler dla ścieżek z parametrami {id}:
async fn pobierz_towar(
    State(stan): State<Arc<Mutex<AppState>>>,
    Path(szukane_id): Path<i32>,
) -> Result<Json<Towar>, String> {
    let sejf = stan.lock().unwrap();
    let znaleziony = sejf.magazyn.iter().find(|towar| towar.id == szukane_id);
    match znaleziony {
        Some(towar) => Ok(Json(towar.clone())),
        None => Err(format!("Towar o id:{} nie istnieje", szukane_id)),
    }
}

