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
        // self.allowed_guesses.contains(&word.to_string()) || self.answers.contains(&word.to_string())
        // TODO: use binary search as the lists are already sorted alphabetically
        self.binary_search(word, &self.answers) || self.binary_search(word, &self.allowed_guesses)
    }

    fn binary_search(&self, word: &str, list: &Vec<String>) -> bool {
        let characters: Vec<char> = word.chars().collect();
        let mut start = 0;
        let mut end = list.len() - 1;
        while start <= end {
            let mut mid = start + (end - start) / 2;
            let current_mid_word: Vec<char> = list[mid].chars().collect();
            if word == list[mid] {
                return true;
            }
            let mut character_index = 0; // which character to compare
            while characters[character_index] == current_mid_word[character_index] {
                // current character matches but not the same word so check the next character
                character_index += 1;
            }
            if characters[character_index] > current_mid_word[character_index] {
                start = mid + 1;
            } else {
                end = mid - 1;
            }
        }
        false
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
