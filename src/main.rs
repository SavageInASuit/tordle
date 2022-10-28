use rocket::{get, post, launch, routes};
use rocket::serde::{Serialize, Deserialize, json::Json};
use rocket::fs::FileServer;

#[derive(Serialize)]
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

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct UserData {
    name: String,
    last: Option<[MarkedGuess; 6]>,
}

#[post("/makeGuess", data = "<guess>")]
fn make_guess(guess: Json<Guess>) -> Json<MarkedGuess> {
    let Json(parsed_guess) = guess;
    Json(MarkedGuess{guess: parsed_guess, marks: [GuessState::Hit, GuessState::Hit, GuessState::Hit, GuessState::Hit, GuessState::Hit]})
}

#[get("/userData")]
fn user_data() -> Json<UserData> {
    Json(UserData{
        name: "Tester".to_owned(), 
        last: Some(
            [
            MarkedGuess{
                guess: Guess{letters: ['b', 'r', 'o', 'k', 'e']},
                marks: [GuessState::Miss, GuessState::Hit, GuessState::Letter, GuessState::Miss, GuessState::Miss],
            },
            MarkedGuess{
                guess: Guess{letters: ['p', 'r', 'o', 'k', 'e']},
                marks: [GuessState::Miss, GuessState::Hit, GuessState::Letter, GuessState::Miss, GuessState::Miss],
            },
            MarkedGuess{
                guess: Guess{letters: ['p', 'z', 'z', 'z', 'z']},
                marks: [GuessState::Miss, GuessState::Miss, GuessState::Miss, GuessState::Miss, GuessState::Miss],
            },
            MarkedGuess{
                guess: Guess{letters: ['p', 'z', 'z', 'z', 'z']},
                marks: [GuessState::Miss, GuessState::Miss, GuessState::Miss, GuessState::Miss, GuessState::Miss],
            },
            MarkedGuess{
                guess: Guess{letters: ['p', 'z', 'z', 'z', 'z']},
                marks: [GuessState::Miss, GuessState::Miss, GuessState::Miss, GuessState::Miss, GuessState::Miss],
            },
            MarkedGuess{
                guess: Guess{letters: ['p', 'z', 'z', 'z', 'z']},
                marks: [GuessState::Miss, GuessState::Miss, GuessState::Miss, GuessState::Miss, GuessState::Miss],
            },
            ]
        )
    })
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", FileServer::from("public"))
        .mount("/", routes![user_data, make_guess])
}
