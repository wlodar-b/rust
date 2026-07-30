-- Add up migration script here
-- Tworzymy tabelę 'towary' w bazie danych
CREATE TABLE IF NOT EXISTS towary (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    nazwa TEXT NOT NULL,
    marka TEXT NOT NULL,
    kolor TEXT NOT NULL,
    rozmiar INTEGER NOT NULL,
    stan TEXT NOT NULL,
    cena_zakupu REAL NOT NULL,
    cena_sprzedazy REAL NOT NULL
);