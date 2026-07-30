# WMS – Warehouse Management System (Rust)

Prosty, ale wydajny system WMS napisany w języku Rust. Projekt składa się z dwóch niezależnych wersji, obrazujących ewolucję aplikacji od lokalnego narzędzia konsolowego (CLI) po pełnoprawną usługę sieciową (REST API).

---

## 📂 Struktura Projektu: WMS vs WMS_api

Projekt został podzielony na dwa katalogi, które różnią się architekturą, sposobem interakcji z użytkownikiem oraz zarządzaniem pamięcią:

| Cecha | `WMS/` (Wersja 1.0 - CLI) | `WMS_api/` (Wersja 2.0 - REST API) |
| :--- | :--- | :--- |
| **Interfejs** | Interaktywne menu konsolowe (CLI) | Serwer sieciowy HTTP (JSON / REST API) |
| **Framework** | Czysty Rust (Standard Library) | **Axum 0.8** + **Tokio** (Asynchroniczność) |
| **Pamięć i Wątki** | Jednowątkowa obsługa w pętli głównej | Wielowątkowy stan bezpieczny (`Arc<Mutex<AppState>>`) |
| **Trwałość danych** | Automatyczny zapis/odczyt z pliku `baza_towaru.json` | Obecnie in-memory (RAM) z planowaną migracją do bazy SQL/JSON |
| **Obsługiwane operacje** | Pełny CRUD, raporty marży, wyszukiwarka w konsoli | Pełny standard **CRUD** przez metody HTTP (`GET`, `POST`, `PUT`, `DELETE`) |

---

## 💻 1. Folder `WMS/` – Lokalny System Konsolowy (CLI)

Klasyczna wersja programu uruchamiana w terminalu, idealna do pracy jednostanowiskowej:
* **Księgowanie i wydawanie towaru:** Szybkie dodawanie przedmiotów (nazwa, marka, rozmiar, stan `A-D`) oraz ich usuwanie z wyliczeniem marży.
* **Zapis i Odczyt (JSON):** Aplikacja automatycznie ładuje i zapisuje cały stan magazynowy do pliku `baza_towaru.json`.
* **Inteligentne ID i Kuloodporność:** System zapobiega duplikatom identyfikatorów i nie wyłącza się przy wprowadzaniu błędnych formatów danych.
* **Architektura:** Podział na `main.rs` (interfejs/menu), `towar.rs` (definicje modeli) oraz `operacje.rs` (logika biznesowa).

---

## 🌐 2. Folder `WMS_api/` – Asynchroniczny Serwer REST API (Axum)

Nowoczesna odsłona systemu, umożliwiająca zarządzanie magazynem z poziomu sieci (np. przez Thunder Client, Postman lub przyszłą aplikację frontendową):
* **Pełny standard CRUD (REST):**
  * `POST /dodaj` – Dodawanie nowego towaru w formacie JSON.
  * `GET /magazyn` – Pobieranie pełnej listy asortymentu.
  * `GET /magazyn/{id}` – Wyszukiwanie konkretnego towaru po jego identyfikatorze.
  * `PUT /edytuj/{id}` – Edycja parametrów istniejącego towaru (cena, stan techniczny itp.).
  * `DELETE /usun/{id}` – Wydawanie (usuwanie) towaru z magazynu.
* **Bezpieczeństwo współbieżności:** Wykorzystanie inteligentnych wskaźników i blokad (`Arc<Mutex<AppState>>`) do bezpiecznej modyfikacji danych przez wiele jednoczesnych żądań HTTP.

---

## 📅 Plany na dalszy rozwój (Wersja 3.0)

1. **Trwałość danych w `WMS_api`:** Podpięcie zapisu stanu magazynu do pliku JSON (na wzór wersji CLI) lub integracja z prawdziwą relacyjną bazą danych (**SQLite** / **PostgreSQL**).
2. **Refaktoryzacja API:** Rozbicie pliku `main.rs` w `WMS_api` na mniejsze, tematyczne moduły (`handlery.rs`, `router.rs`).
3. **Graficzny Interfejs Użytkownika (Web GUI):** Stworzenie interfejsu frontendowego (np. w Next.js/TypeScript), który połączy się z naszym Rustowym backendem.
4. **Wdrożenie Sieciowe:** Konteneryzacja gotowego serwera i uruchomienie w sieci lokalnej jako usługi działającej w trybie 24/7.