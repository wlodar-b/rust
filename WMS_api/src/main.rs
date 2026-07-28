use axum::{routing::get, Router};

#[tokio::main]
async fn main() {
    // Tworzenie głównego routera aplikacji
    let app = Router::new()
        .route("/", get(|| async { "Witaj w WMS 2.0 (REST API)!"}));

    // Konfigurujemy port na którym serwer będzie nasłuchiwał
    let port = 3000;
    println!("Serwer WMS 2.0 wystartował na porcie: {}", port);

    // Uruchamiamy właściwy nasłuch sieciowy
    let listener = tokio::net;;TcpListener::bind(format!("0.0.0.0:{}", port)).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}