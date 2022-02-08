use colored::Colorize;

mod history;
mod input;
mod visuals;
mod words;
use history::{GameHistory, GameState};
use input::InputsValidator;
use words::Words;

fn main() {
    let words = Words::load();
    let answer = words.get_random_answer();

    let mut history = GameHistory::new(String::from(answer));

    let mut input_validator = InputsValidator::new(&words);

    loop {
        visuals::clear_screen();

        println!("answer:\t {}", answer);

        println!("{}", history.draw_history());

        // check if there was a problem on last input and display message
        let last_validity_issue = input_validator.get_validity_issue();
        if !last_validity_issue.is_empty() {
            println!("{}", last_validity_issue.red());
            input_validator.clear_issue();
        }

        // receive user input and validate
        println!("Pleast input your 5 letter answer");
        let guess = input_validator.read_input();

        if !input_validator.validity_issue.is_none() {
            continue;
        }

        let state = history.guess(guess);

        match state {
            GameState::OnGoing => continue,
            _ => {
                visuals::clear_screen();
                println!("{}", history.draw_history());
                match state {
                    GameState::Lost => {
                        println!(
                            "{} {}",
                            "Lost! The correct answer was".red(),
                            answer.white().on_red()
                        );
                    }
                    GameState::Won => {
                        println!(
                            "{} {}",
                            "Congrats! The correct answer is".green(),
                            answer.black().on_green()
                        );
                    }
                    _ => (),
                }
                break;
            }
        }
    }
}
