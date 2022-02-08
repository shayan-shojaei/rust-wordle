use colored::*;
use rand::{thread_rng, Rng};
use std::fs;
use std::io::stdin;

fn main() {
    // load word lists
    let answers_list = read_words_from_file("answers.txt");
    let allowed_guesses = read_words_from_file("allowed-guesses.txt");

    // a vector of strings holding each 5-letter guess the user has input
    let mut guesses_history: Vec<String> = Vec::new();
    // game answer picked randomly from 'answers_list' vector
    let answer = get_random_word(&answers_list);
    // game iteration
    let mut round: u8 = 0;

    /*
    validate user input
    hold problem message if there is any
    otherwise set to none
    */
    let mut last_problem: Option<String> = None;
    loop {
        clear_screen();

        // println!("answer:\t {}", answer);

        // draw game history
        println!("{}", draw_guess_history(&answer, &guesses_history));

        // check if there was a problem on last input and display message
        if !last_problem.is_none() {
            println!("{}", last_problem.unwrap().red());
        }

        // receive user input
        println!("Pleast input your 5 letter answer");
        let mut guess = String::new();
        stdin()
            .read_line(&mut guess)
            .expect("There was an issue with the input");
        guess = guess.trim().to_string();

        // validate input
        let validation = validate_input(&guess, &allowed_guesses, &answers_list);

        // set 'last_problem' to validation problem or reset if validation is ok
        match validation {
            Validation::Problem(message) => {
                last_problem = Some(message);
                continue;
            }
            _ => last_problem = None,
        }

        // if answer was correctly guessed
        let won = guess == answer;

        guesses_history.push(guess);
        round += 1;

        // game ends after 6 guesses or correct guess
        if round == 6 || won {
            clear_screen();
            println!("{}", draw_guess_history(&answer, &guesses_history));

            // output message to show the answer
            let message = match won {
                true => format!(
                    "{} '{}'",
                    "Congrats! The correct answer was".green(),
                    answer.green()
                ),
                false => format!(
                    "{} '{}'",
                    "Failed! The correct answer was".red(),
                    answer.red()
                ),
            };
            println!("{}", message);
            break;
        }
    }
}

// clear terminal screen and set the cursor on the first line and first character
fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
}

// open file and create a vector of strings with each line of the file as its elements
fn read_words_from_file(filename: &str) -> Vec<String> {
    let file = match fs::read_to_string(filename) {
        Ok(text) => text,
        _ => return vec![],
    };

    let words: Vec<String> = file.lines().map(String::from).collect();
    words
}

// choose a random element of vector
fn get_random_word(list: &Vec<String>) -> String {
    let mut rand = thread_rng();
    let random_index = rand.gen_range(0..list.len());
    list[random_index].to_string()
}

enum Validation {
    Ok,
    Problem(String),
}

fn validate_input(
    input: &String,
    allowed_guesses: &Vec<String>,
    answers: &Vec<String>,
) -> Validation {
    if input.len() != 5 {
        // wrong input size
        return Validation::Problem(String::from("The input should be a 5 letter English word!"));
    } else if !allowed_guesses.contains(input) && !answers.contains(input) {
        // words list
        return Validation::Problem(String::from("Not a valid word!"));
    }
    Validation::Ok
}

fn draw_guess_history(answer: &str, guesses: &Vec<String>) -> String {
    let mut history = String::new();

    let answer: Vec<char> = answer.chars().collect();

    // for each of the possible 6 rounds go through the history
    for round in 0..6 {
        if round >= guesses.len() {
            // not played (yet) -> set all to empty gray blocks
            let empty_box = gen_box(&' ', State::Unfilled);
            history.push_str(format!("{0}{0}{0}{0}{0}", empty_box).as_str());
        } else {
            // played -> check each character against the answer characters
            for (i, c) in guesses[round].chars().enumerate() {
                let current_box: String;
                if c == answer[i] {
                    // correct letter and position
                    current_box = gen_box(&c, State::Correct);
                } else if answer.contains(&c) {
                    // correct letter but incorrect position
                    current_box = gen_box(&c, State::Contained);
                } else {
                    // incorrect letter
                    current_box = gen_box(&c, State::Wrong);
                }
                history.push_str(current_box.as_str());
            }
        }
        if round < 5 {
            history.push('\n');
        }
    }

    history
}

enum State {
    Unfilled,
    Wrong,
    Contained,
    Correct,
}

fn gen_box(c: &char, state: State) -> String {
    match state {
        State::Unfilled => format!("{}", c.to_string().on_bright_black()).to_string(),
        State::Wrong => format!("{}", c.to_string().white().on_red().bold()).to_string(),
        State::Contained => format!("{}", c.to_string().black().on_yellow().bold()).to_string(),
        State::Correct => format!("{}", c.to_string().black().on_bright_green().bold()).to_string(),
    }
}
