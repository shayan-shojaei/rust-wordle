use crate::words::Words;
use std::io::stdin;

pub struct InputsValidator<'a> {
    pub validity_issue: Option<String>,
    words: &'a Words,
}

impl<'a> InputsValidator<'a> {
    pub fn new(words: &'a Words) -> InputsValidator<'a> {
        InputsValidator {
            validity_issue: None,
            words,
        }
    }

    pub fn read_input(&mut self) -> String {
        let mut input = String::new();
        stdin()
            .read_line(&mut input)
            .expect("There was an issue with the input");
        input = input.trim().to_lowercase().to_string();
        self.validity_issue = self.validate_input(&input);
        input
    }

    pub fn get_validity_issue(&self) -> &str {
        match &self.validity_issue {
            None => "",
            Some(issue) => issue.as_str(),
        }
    }

    pub fn clear_issue(&mut self) {
        self.validity_issue = None;
    }

    pub fn validate_input(&self, input: &str) -> Option<String> {
        if input.len() != 5 {
            return Some("The input should be a 5 letter English word!".to_string());
        } else if !self.words.contains(input) {
            return Some("Not a valid word!".to_string());
        }
        None
    }
}
