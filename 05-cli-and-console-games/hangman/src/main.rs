use rand::seq::SliceRandom;
use std::io;

fn main() {
    struct PlayerRoot {
        word: String,
        no_of_guesses: i8,
        output_string: Vec<char>,
        max_tries: i8,
        correct_guesses: Vec<char>,
    }

    impl PlayerRoot {
        fn new(word: &str, max_tries: i8) -> PlayerRoot {
            PlayerRoot {
                word: String::from(word),
                no_of_guesses: 0,
                output_string: vec!['_'; word.chars().count()],
                max_tries,
                correct_guesses: Vec::new(),
            }
        }

        fn generate_random_word(list: &[String]) -> String {
            let word = list.choose(&mut rand::thread_rng()).unwrap();
            println!("word {:?}", word);
            word.to_string()
        }
    }

    //list of words for the game
    let list_of_words = vec![
        "hunting".to_string(),
        "dizzy".to_string(),
        "while".to_string(),
        "string".to_string(),
        "something".to_string(),
        "notified".to_string(),
    ];

    let random_word = PlayerRoot::generate_random_word(&list_of_words);

    // for our simple UI
    let word_length = random_word.chars().count();
    let mut player_one = PlayerRoot::new(&random_word, 10);

    println!("Welcome to the hangman game built with rust!, please enter a letter");
    println!(
        "{:?} [remaining guesses: {:?}, max tries {:?}]",
        player_one.output_string, player_one.no_of_guesses, player_one.max_tries
    );

    loop {
        //Takes in an input
        //Todo Check if input is more than one char
        let mut guess = String::from("");
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        let altered_guess: char = match guess.trim().chars().next() {
            Some(val) => val,
            _ => {
                println!("No letter inputted,type a letter!");
                //It complains it needs a char :D
                '0'
            }
        };

        // Checks if guess is valid
        if !player_one.output_string.contains(&altered_guess) {
            player_one.no_of_guesses += 1;

            if !player_one.word.contains(altered_guess) {
                let guess_score = player_one.max_tries - player_one.no_of_guesses;
                println!(
                    "Wrong guess 🫨\n{:?} [remaining guesses: {:?}]",
                    player_one.output_string, guess_score
                );
            }

            // loops through the word, check if guess is correct, reduces number of guess by one
            for n in player_one.word.char_indices() {
                if n.1 == altered_guess {
                    let guess_score = player_one.max_tries - player_one.no_of_guesses;

                    player_one.correct_guesses.push(n.1);
                    player_one.output_string[n.0] = n.1;
                    println!(
                        "{:?} [remaining guesses: {:?}]",
                        player_one.output_string, guess_score
                    );
                }
            }
        } else {
            println!("That letter is taken!!! guess again")
        }

        // If the player wins
        if player_one.correct_guesses.len() == word_length {
            println!("YOU WIN!!");
            break;
        }

        // If the player loses
        if player_one.max_tries == player_one.no_of_guesses {
            println!("GAME OVER!!! \n THE WORD IS {:?}", player_one.word);
            break;
        }
    }
}
