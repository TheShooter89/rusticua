use std::{fs::File, io::Read};

use rand::Rng;

use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Letter {
    pub character: String,
    pub lowercase: String,
    pub latin_transliteration: String,
    pub italian_pronunciation: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportedLetters {
    alphabet: Vec<Letter>,
}

pub fn get_random_letters() -> Vec<Letter> {
    let mut json_content = String::new();
    let mut file = File::open("static/data/ua_alphabet.json").expect("error opening json file");
    file.read_to_string(&mut json_content)
        .expect("error reading JSON file");

    let imported_letters: ImportedLetters =
        serde_json::from_str(&json_content).expect("error parsing json");

    println!("imported_letters: {:?}", imported_letters);
    println!(
        "imported_letters.length(): {:?}",
        imported_letters.alphabet.len()
    );

    let result = pick_multiple_random_elements(imported_letters.alphabet, 3);
    result
}

pub fn pick_multiple_random_elements<T: Clone + std::fmt::Debug>(
    source: Vec<T>,
    iterations: usize,
) -> Vec<T> {
    let mut rng = rand::thread_rng();
    let mut unique_index = Vec::new();
    let mut final_elements: Vec<T> = Vec::new();

    while unique_index.len() < iterations {
        let random_num = rng.gen_range(0..source.len());

        if !unique_index.contains(&random_num) {
            unique_index.push(random_num);
            final_elements.push(source[random_num].clone());
        }
    }
    final_elements
}
