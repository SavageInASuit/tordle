#[cfg(test)]
mod test {
    use crate::{Guess, mark_guess, GuessState, MarkedGuess, get_todays_word};
    use rocket::local::blocking::Client;
    use rocket::http::Status;

    struct MarkedGuessTestCase {
        word: String,
        marked_guess: MarkedGuess,
    }

    fn get_marked_guesses() -> [MarkedGuessTestCase; 5] {
        [
            MarkedGuessTestCase {
                word: "guess".to_string(),
                marked_guess: MarkedGuess {
                    guess: Guess { letters: ['e', 'e', 'e', 'e', 'e'] },
                    marks: [GuessState::Miss, GuessState::Miss, GuessState::Hit, GuessState::Miss, GuessState::Miss],
                }
            },
            MarkedGuessTestCase {
                word: "guess".to_string(),
                marked_guess: MarkedGuess {
                    guess: Guess { letters: ['w', 'r', 'o', 'n', 'g'] },
                    marks: [GuessState::Miss, GuessState::Miss, GuessState::Miss, GuessState::Miss, GuessState::Letter],
                }
            },
            MarkedGuessTestCase {
                word: "guess".to_string(),
                marked_guess: MarkedGuess {
                    guess: Guess { letters: ['w', 'w', 'w', 'w', 'w'] },
                    marks: [GuessState::Miss, GuessState::Miss, GuessState::Miss, GuessState::Miss, GuessState::Miss],
                }
            },
            MarkedGuessTestCase {
                word: "guess".to_string(),
                marked_guess: MarkedGuess {
                    guess: Guess { letters: ['g', 'u', 'e', 's', 's'] },
                    marks: [GuessState::Hit, GuessState::Hit, GuessState::Hit, GuessState::Hit, GuessState::Hit],
                }
            },
            MarkedGuessTestCase {
                word: "toast".to_string(),
                marked_guess: MarkedGuess {
                    guess: Guess { letters: ['g', 'r', 'o', 't', 't'] },
                    marks: [GuessState::Miss, GuessState::Miss, GuessState::Letter, GuessState::Letter, GuessState::Hit],
                }
            },
        ]
    }

    #[test]
    fn guesses_marked_correctly() {
        let test_cases = get_marked_guesses();
        for test_case in test_cases {
            let marked_guess = mark_guess(test_case.marked_guess.guess, &test_case.word);
            let marks = marked_guess.marks;

            assert_eq!(
                marks, 
                test_case.marked_guess.marks
            );
        }
    }

    #[test]
    fn guess_good_request_okay_response() {
        let client = Client::tracked(crate::rocket()).expect("valid `rocket`");

        let response = client.post("/makeGuess").body("{\"letters\": [\"a\",\"b\",\"c\",\"d\",\"e\"]}").dispatch();

        assert_eq!(response.status(), Status::Ok);
    }

    #[test]
    fn guess_bad_request_bad_request_response() {
        let client = Client::tracked(crate::rocket()).expect("valid `rocket`");

        let response = client.post("/makeGuess").body("{\"letters\": [\"a\"]}").dispatch();

        assert_eq!(response.status(), Status::UnprocessableEntity);
    }

    #[test]
    fn todays_word_valid() {
        let word_of_the_day = get_todays_word();

        assert!(word_of_the_day.len() == 5);
    }
}