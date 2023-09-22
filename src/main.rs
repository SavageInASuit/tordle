pub mod tests;

use chrono::{Datelike, Utc};
use std::collections::HashMap;

use rocket::fs::FileServer;
use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket::{launch, post, routes};

#[derive(Serialize, PartialEq, Debug)]
#[serde(crate = "rocket::serde")]
enum GuessState {
    Miss,
    Letter,
    Hit,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct Guess {
    letters: [char; 5],
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct MarkedGuess {
    guess: Guess,
    marks: [GuessState; 5],
}

// Vector of words to guess
const WORDS: [&str; 30] = [
    "quote", "drive", "clock", "strap", "guess", "sting", "speak", "thine", "board", "chuck",
    "toast", "cross", "sugar", "quilt", "jelly", "honey", "uncle", "mouse", "lunch", "atlas",
    "navel", "glove", "piano", "earth", "water", "fruit", "dough", "knife", "virus", "bevel",
];

// Get index for date
fn get_index_for_date(date: chrono::DateTime<Utc>) -> usize {
    let day_of_year = date.day() as usize;
    let year = date.year() as usize;
    (day_of_year + year * 1000) % WORDS.len()
}

// Get today's date and use it to get a word from the WORDS vector
fn get_todays_word() -> String {
    WORDS[get_index_for_date(Utc::now())].to_string()
}

fn in_word(c: &char, word: Vec<char>) -> bool {
    word.contains(c)
}

fn mark_guess(guess: Guess, word: &str) -> MarkedGuess {
    let word_chars: Vec<char> = word.chars().collect();
    let mut hits: HashMap<char, u8> = HashMap::new();
    let mut counts: HashMap<char, u8> = HashMap::new();
    let mut marks: [GuessState; 5] = [
        GuessState::Miss,
        GuessState::Miss,
        GuessState::Miss,
        GuessState::Miss,
        GuessState::Miss,
    ];

    // First pass for hits
    for (ind, c) in guess.letters.iter().enumerate() {
        if c == &word_chars[ind] {
            marks[ind] = GuessState::Hit;
            let hit_count = match hits.get(c) {
                Some(count) => count.to_owned(),
                None => 0,
            };
            hits.insert(*c, hit_count + 1);
        }
        counts.insert(
            *c,
            u8::try_from(word_chars.iter().filter(|&o| o == c).count())
                .expect("trying to convert from usize to u8"),
        );
    }
    // Second pass for letters
    for (ind, c) in guess.letters.iter().enumerate() {
        if c != &word_chars[ind]
            && in_word(c, word.chars().collect())
            && (!hits.contains_key(c) || hits[c] != counts[c])
        {
            marks[ind] = GuessState::Letter;
            let hit_count = match hits.get(c) {
                Some(count) => count.to_owned(),
                None => 0,
            };
            hits.insert(*c, hit_count + 1);
        }
    }
    MarkedGuess { guess, marks }
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct UserData {
    name: String,
    last: Option<[MarkedGuess; 6]>,
}

#[post("/makeGuess", data = "<guess>")]
fn make_guess(guess: Json<Guess>) -> Json<MarkedGuess> {
    let Json(parsed_guess) = guess;
    let word_of_the_day = get_todays_word();
    Json(mark_guess(parsed_guess, &word_of_the_day))
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", FileServer::from("public"))
        .mount("/", routes![make_guess])
}
