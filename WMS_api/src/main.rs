mod towar;
use axum::{routing::get, Router, extract::State, Json};
use towar::Towar;
use std::sync::{Arc, Mutex};

struct AppState { 
    magazyn: Vec<Towar>,
    aktualne_id: i32,
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
        .with_state(stan_aplikacji); // podpinamy stan aplikacji do routera

    // Konfigurujemy port na którym serwer będzie nasłuchiwał
    let port = 3000;
    println!("Serwer WMS 2.0 wystartował na porcie: {}", port);

    // Uruchamiamy właściwy nasłuch sieciowy
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port)).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn pobierz_magazyn(State(stan): State<Arc<Mutex<AppState>>>) -> Json<Vec<Towar>> {
    // Otwieramy sejf
    let sejf = stan.lock().unwrap();
    // Klonujemy zawartość wektora z magazynem
    let kopia_magazynu = sejf.magazyn.clone();
    // Pakujemy sklonowane dane do Json i zwracamy
    Json(kopia_magazynu)
}