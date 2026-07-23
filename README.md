# WMS 

Prosty, ale wydajny system WMS (Warehouse Management System) uruchamiany w konsoli, napisany w języku Rust. Aplikacja została stworzona z myślą o szybkiej inwentaryzacji i zarządzaniu asortymentem.

---

## 🚀 Funkcjonalności

*   **Księgowanie towaru:** Szybkie dodawanie nowych przedmiotów do bazy z uwzględnieniem nazwy, marki oraz rozmiaru.
*   **Bezpieczne stany (Enum):** Ścisła kontrola jakości towaru za pomocą zdefiniowanych stanów (`A`, `B`, `C`, `D`) opartych na typie wyliczeniowym, co eliminuje błędy przy wprowadzaniu danych.
*   **Wyszukiwarka:** Wielopoziomowe menu pozwalające na filtrowanie aktualnego asortymentu po marce, rozmiarze lub stanie.
*   **Wydawanie towaru:** Usuwanie sprzedanych przedmiotów ze stanu magazynowego na podstawie ich unikalnego numeru ID.
*   **Zapis i Odczyt (JSON):** Aplikacja automatycznie wczytuje i zapisuje cały stan magazynowy do pliku `baza_towaru.json`, chroniąc dane przed utratą po zamknięciu programu.
*   **Inteligentne ID:** System analizuje najwyższe użyte ID przy starcie programu, zapobiegając duplikatom po usunięciu części asortymentu.

---

## 🏗️ Architektura i Struktura Projektu

Aplikacja wykorzystuje bibliotekę `serde` do serializacji i deserializacji danych. Kod został podzielony na moduły w celu zachowania czystości i skalowalności:

*   `main.rs` – Główna pętla programu, interfejs użytkownika (CLI) oraz logika operacji na wektorze.
*   `towar.rs` – Definicja struktury `Towar` oraz typu wyliczeniowego `StanTowaru`.

---

## 📅 Plany na przyszły rozwój (To Do)

1.  **Refaktoryzacja Architektury:** Wydzielenie logiki biznesowej (dodawanie, wyszukiwanie, usuwanie) z pliku `main.rs` do osobnych, dedykowanych funkcji.
2.  **Kuloodporna obsługa błędów:** Wyeliminowanie użycia `expect()` przy parsowaniu danych liczbowych na rzecz bezpiecznej obsługi błędów za pomocą wyrażeń `match`, aby program nie zamykał się przy literówkach użytkownika.
3.  **Moduł Finansowy (Marża):** Rozbudowa struktury o cenę zakupu oraz sprzedaży w celu automatycznego wyliczania zysku z usuniętych (sprzedanych) produktów.
