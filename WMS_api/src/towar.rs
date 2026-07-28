use serde::{Serialize, Deserialize}; // Ten plik też musi wiedzieć, co to Serialize

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum StanTowaru {  
    A,
    B,
    C,
    D,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Towar {     
    pub nazwa: String, 
    pub marka: String,
    pub kolor: String,
    pub rozmiar: i32,
    pub stan: StanTowaru,
    pub id: i32,
    pub cena_zakupu: f32,
    pub cena_sprzedazy: f32,
}

