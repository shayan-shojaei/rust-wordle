use rand::{thread_rng, Rng};
use std::fs;
pub struct Words {
    answers: Vec<String>,
    allowed_guesses: Vec<String>,
}

impl Words {
    pub fn load() -> Words {
        let answers = Words::read_words_from_file("answers.txt");
        let allowed_guesses = Words::read_words_from_file("allowed-guesses.txt");
        Words {
            answers,
            allowed_guesses,
        }
    }

    pub fn contains(&self, word: &str) -> bool {
        // TODO: use binary search as the lists are already sorted alphabetically
        self.allowed_guesses.contains(&word.to_string()) || self.answers.contains(&word.to_string())
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
    pub fn get_random_answer(&self) -> &str {
        let mut rand = thread_rng();
        let random_index = rand.gen_range(0..self.answers.len());
        self.answers[random_index].as_str()
    }
}
