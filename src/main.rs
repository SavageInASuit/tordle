pub mod tests;

use std::collections::HashMap;
use chrono::{Utc, Datelike};

use rocket::http::{Cookie, CookieJar};
use rocket::{get, post, launch, routes};
use rocket::serde::{Serialize, Deserialize, json::Json};
use rocket::fs::FileServer;

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


fn get_today_formatted() -> String {
    let now = Utc::now();
    format!("{}-{}-{}", now.year(), now.month(), now.day())
}

fn get_todays_word() -> String {
    let mut words: HashMap<String, String> = HashMap::new();
    words.insert("2022-11-17".to_string(), "guess".to_string());
    words.insert("2022-11-18".to_string(), "sting".to_string());
    words.insert("2022-11-19".to_string(), "speak".to_string());
    words.insert("2022-11-20".to_string(), "thine".to_string());
    words.insert("2022-11-21".to_string(), "board".to_string());
    words.insert("2022-11-22".to_string(), "hoard".to_string());
    words.insert("2022-11-23".to_string(), "grove".to_string());
    words.insert("2022-11-24".to_string(), "plonk".to_string());
    words.insert("2022-11-25".to_string(), "wrong".to_string());
    words.insert("2022-11-26".to_string(), "zebra".to_string());
    words.insert("2022-11-27".to_string(), "quote".to_string());
    words.insert("2022-11-28".to_string(), "drive".to_string());
    words.insert("2022-11-29".to_string(), "clock".to_string());
    words.insert("2022-11-30".to_string(), "strap".to_string());
    let today = get_today_formatted();
    
    match words.get(&today) {
        Some(word) => word.to_owned(),
        None => "".to_owned(),
    }
}

fn in_word(c: &char, word: Vec<char>) -> bool {
    word.contains(c)
}

fn mark_guess(guess: Guess, word: &str) -> MarkedGuess {
    let word_chars: Vec<char> = word.chars().collect();
    let mut hits: HashMap<char, u8> = HashMap::new();
    let mut counts: HashMap<char, u8> = HashMap::new();
    let mut marks: [GuessState; 5] = [GuessState::Miss, GuessState::Miss, GuessState::Miss, GuessState::Miss, GuessState::Miss];

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
                .expect("trying to convert from usize to u8")
        );
    }
    // Second pass for letters
    for (ind, c) in guess.letters.iter().enumerate() {
        if c != &word_chars[ind] && in_word(c, word.chars().collect()) && (!hits.contains_key(c) || hits[c] != counts[c]) {
            marks[ind] = GuessState::Letter;
            let hit_count = match hits.get(c) {
                Some(count) => count.to_owned(),
                None => 0,
            };
            hits.insert(*c, hit_count + 1);
        }
    }
    MarkedGuess{guess, marks}
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

fn get_user_data(cookie: &Cookie) -> UserData {
    println!("Cookie is {:?}", cookie);
    UserData{
        name: "Tester".to_owned(), 
        last: None,
    }
}

#[get("/userData")]
fn user_data(cookies: &CookieJar<'_>) -> Json<UserData> {
    let user_data = match cookies.get("tordle-user") {
        Some(cookie) => get_user_data(cookie),
        None => UserData{
            name: "New User".to_owned(), 
            last: None,
        },
    };
    Json(user_data)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", FileServer::from("public"))
        .mount("/", routes![user_data, make_guess])
}
