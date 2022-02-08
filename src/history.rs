use colored::*;

#[derive(Copy, Clone, Debug)]
pub enum TileState {
    Unfilled(char),
    Wrong(char),
    Contained(char),
    Correct(char),
}

pub enum GameState {
    OnGoing,
    Won,
    Lost,
}

pub struct GameHistory {
    answer: String,
    answer_characters: Vec<char>,
    guesses: Vec<String>,
    pub board: Vec<Vec<TileState>>,
    round: usize,
}

impl GameHistory {
    pub fn new(answer: String) -> GameHistory {
        let board = vec![vec![TileState::Unfilled(' '); 5]; 6];
        GameHistory {
            answer_characters: answer.chars().collect(),
            answer,
            guesses: Vec::new(),
            round: 0,
            board,
        }
    }

    /*
    check each character in guess against corresponding character in answer
    if the character in that position is the same as the character in the same position of answer
    it's a correct answer -> CORRECT
    if the character in that position is contained within the answer
    it's not the correct position -> CONTAINED
    any other case is an incorrect guess -> WRONG

    increment round number
    at the end return game state
     */
    pub fn guess(&mut self, word: String) -> GameState {
        for (index, character) in word.chars().enumerate() {
            let current_box: TileState;
            if character == self.answer_characters[index] {
                current_box = TileState::Correct(character);
            } else if self.answer.contains(character) {
                current_box = TileState::Contained(character);
            } else {
                current_box = TileState::Wrong(character);
            }
            self.board[self.round][index] = current_box;
        }
        self.guesses.push(word.clone());
        self.round += 1;

        if self.answer == word {
            return GameState::Won;
        } else if self.round == self.board.len() {
            return GameState::Lost;
        }
        GameState::OnGoing
    }

    pub fn draw_history(&self) -> String {
        let mut history_text = String::new();

        for (i, round) in self.board.iter().enumerate() {
            for tile_state in round {
                history_text.push_str(GameHistory::gen_box(tile_state).as_str());
            }
            if i < self.board.len() - 1 {
                history_text.push('\n');
            }
        }

        history_text
    }
    fn gen_box(state: &TileState) -> String {
        match state {
            TileState::Unfilled(letter) => {
                format!("{}", letter.to_string().on_bright_black())
            }
            TileState::Wrong(letter) => {
                format!("{}", letter.to_string().white().on_red().bold())
            }
            TileState::Contained(letter) => {
                format!("{}", letter.to_string().black().on_yellow().bold())
            }
            TileState::Correct(letter) => {
                format!("{}", letter.to_string().black().on_bright_green().bold())
            }
        }
    }
}
