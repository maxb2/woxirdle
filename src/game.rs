use colored::{ColoredString, Colorize};
use rand::seq::SliceRandom;
use std::fmt::Formatter;
use std::{error::Error, fmt};

#[derive(Debug, Clone)]
pub struct Dictionary<'a> {
    pub words: Vec<&'a str>,
}

impl<'a> Dictionary<'a> {
    pub fn is_valid_word(&self, word: &str) -> bool {
        self.words.iter().any(|&i| i == word)
    }
    pub fn get_random_word(&self) -> Option<&str> {
        self.words.choose(&mut rand::thread_rng()).cloned()
    }
}

#[derive(Debug)]
pub enum LetterState {
    Correct,
    Included,
    Excluded,
}

#[derive(Debug)]
pub struct Turn {
    pub word: String,
    pub states: Vec<LetterState>,
}

#[derive(Debug)]
pub struct TurnError {
    pub message: String,
}

impl Error for TurnError {}

impl fmt::Display for TurnError {
    fn fmt(&self, _: &mut Formatter) -> Result<(), std::fmt::Error> {
        todo!()
    }
}

// impl fmt::Display for Turn<'_> {
//     fn fmt(&self, _: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
//         todo!()
//     }
// }

impl<'a> Turn {
    fn try_new(answer: &str, guess: String) -> Result<Self, TurnError> {
        if guess.len() != answer.len() {
            return Err(TurnError {
                message: String::from("Guess is not the same length as answer!"),
            });
        }

        let mut _states: Vec<LetterState> = vec![];

        let pairs = guess.chars().zip(answer.chars());
        for pair in pairs {
            if pair.0 == pair.1 {
                _states.push(LetterState::Correct)
            } else if answer.chars().any(|c| c == pair.0) {
                _states.push(LetterState::Included)
            } else {
                _states.push(LetterState::Excluded)
            }
        }

        if _states.len() != answer.len() {
            unreachable!()
        }

        Ok(Turn {
            word: guess,
            states: _states,
        })
    }
}

#[derive(Debug)]
pub enum GameStates {
    InProgress,
    Won,
    Lost,
}

#[derive(Debug)]
pub struct Game<'a> {
    _size: usize,
    _state: GameStates,
    answer: String,
    dictionary: &'a Dictionary<'a>,
    turns: Vec<Turn>,
}

impl<'a> Game<'a> {
    pub fn new(answer: String, dictionary: &'a Dictionary) -> Result<Self, ()> {
        if dictionary.is_valid_word(&answer) {
            return Ok(Self {
                _size: answer.len() + 1,
                _state: GameStates::InProgress,
                answer,
                dictionary,
                turns: vec![],
            });
        }
        Err(())
    }

    pub fn state(&self) -> &GameStates {
        &self._state
    }

    pub fn turns(&self) -> &Vec<Turn> {
        return &self.turns;
    }

    pub fn turn_strings(&self) -> Option<String> {
        let turn = self.turns.last()?;

        let letters: String = turn
            .states
            .iter()
            .zip(turn.word.chars())
            .map(|x| match x {
                (LetterState::Correct, c) => format!("{}", c).on_green().to_string(),
                (LetterState::Excluded, c) => format!("{}", c).on_red().to_string(),
                (LetterState::Included, c) => format!("{}", c).on_yellow().to_string(),
            })
            .collect();

        Some(letters)
    }

    pub fn answer(&self) -> &String {
        &self.answer
    }

    pub fn take_turn(&mut self, guess: String) -> Result<(), TurnError> {
        match self._state {
            GameStates::InProgress => {
                if !self.dictionary.words.iter().any(|x| x == &guess) {
                    return Err(TurnError {
                        message: format!("{} is not a valid guess!", guess),
                    });
                }
                let turn = Turn::try_new(&self.answer, guess)?;
                if turn.states.iter().all(|x| match x {
                    LetterState::Correct => true,
                    _ => false,
                }) {
                    self._state = GameStates::Won
                }
                self.turns.push(turn);

                if self.turns.len() >= self._size {
                    self._state = GameStates::Lost
                }

                Ok(())
            }
            _ => Err(TurnError {
                message: String::from("Game is already over!"),
            }),
        }
    }
}
