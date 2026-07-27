# WMS 

Prosty, ale wydajny system WMS (Warehouse Management System) napisany w języku Rust. Aplikacja aktualnie ewoluuje z lokalnego narzędzia konsolowego (CLI) w stronę pełnoprawnej usługi sieciowej.

---

## 🚀 Zrealizowane Funkcjonalności (Wersja 1.0 CLI)

*   **Księgowanie towaru:** Szybkie dodawanie nowych przedmiotów do bazy z uwzględnieniem nazwy, marki oraz rozmiaru.
*   **Bezpieczne stany (Enum):** Ścisła kontrola jakości towaru za pomocą zdefiniowanych stanów (`A`, `B`, `C`, `D`).
*   **Moduł Finansowy:** Automatyczne wyliczanie i raportowanie marży (zysku) podczas wydawania (usuwania) towaru z magazynu na podstawie cen zakupu i sprzedaży.
*   **Wyszukiwarka:** Wielopoziomowe menu pozwalające na filtrowanie aktualnego asortymentu.
*   **Zapis i Odczyt (JSON):** Aplikacja automatycznie wczytuje i zapisuje cały stan magazynowy do pliku `baza_towaru.json`.
*   **Inteligentne ID:** System analizuje najwyższe użyte ID przy starcie programu, zapobiegając duplikatom.
*   **Kuloodporna obsługa wejścia:** Program jest w pełni zabezpieczony przed awaryjnym zamknięciem w przypadku podania błędnego formatu danych.

---

## 🏗️ Aktualna Architektura

Kod został podzielony na moduły w celu zachowania czystości logiki biznesowej:
*   `main.rs` – Główna pętla programu i tekstowy interfejs użytkownika.
*   `towar.rs` – Definicja struktury `Towar` (wzbogacona o ceny) oraz typu wyliczeniowego `StanTowaru`.
*   `operacje.rs` – Centralna logika operacji magazynowych (wyszukiwanie, parsowanie, wyliczanie zysków).

---

## 📅 Plany na przyszły rozwój (Wersja 2.0 - Sieć i Bazy Danych)

1.  **Migracja na REST API (Axum):** Przebudowa architektury na asynchroniczny serwer webowy nasłuchujący żądań HTTP w tle. Zastąpienie pętli konsolowej endpointami.
2.  **Graficzny Interfejs Użytkownika (GUI):** Stworzenie frontendowej aplikacji webowej, która połączy się z naszym serwerem, umożliwiając zarządzanie magazynem z poziomu przeglądarki internetowej.
3.  **Relacyjna Baza Danych (SQL):** Zastąpienie lokalnego pliku JSON profesjonalną bazą danych (np. SQLite) w celu zapewnienia wyższej wydajności, bezpieczeństwa i możliwości wykonywania zaawansowanych zapytań analitycznych.
4.  **Wdrożenie Sieciowe (Homelab):** Konteneryzacja gotowego systemu i uruchomienie go w sieci lokalnej jako usługi działającej w trybie 24/7.