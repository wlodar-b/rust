use serde::{Serialize, Deserialize}; // Ten plik też musi wiedzieć, co to Serialize

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum StanTowaru {  // <-- dodane pub
    A,
    B,
    C,
    D,
}

#[derive(Serialize, Deserialize)]
pub struct Towar {     // <-- dodane pub
    pub nazwa: String, // <-- dodane pub do KAŻDÉGO pola
    pub marka: String,
    pub rozmiar: i32,
    pub stan: StanTowaru,
    pub id: i32,
}