use crossterm::{cursor, terminal, ExecutableCommand, QueueableCommand};
use rand::seq::IteratorRandom;
use serde_json;
use std::{
    fs, io,
    io::{stdout, Write},
};
use woxirdle::game::{Dictionary, Game, GameStates};

const BACKSPACE: char = 8u8 as char;

fn main() -> io::Result<()> {
    let file = fs::File::open("wordle_dictionary.json").expect("file should open read only");
    let json: serde_json::Value =
        serde_json::from_reader(file).expect("file should be proper JSON");

    let answers = Dictionary {
        words: {
            json["answers"]
                .as_array()
                .unwrap()
                .iter()
                .map(|x| x.as_str().unwrap())
                .collect()
        },
    };

    let mut _all: Vec<&str> = json["answers"]
        .as_array()
        .unwrap()
        .iter()
        .chain(json["allowed"].as_array().unwrap().iter())
        .map(|x| x.as_str().unwrap())
        .collect();

    let allowed = Dictionary { words: _all };

    let _answer = answers.words.iter().choose(&mut rand::thread_rng());

    // println!("Answer: {:?}", _answer);

    let mut game = Game::new(String::from(*_answer.unwrap()), &allowed).unwrap();

    let stdin = io::stdin(); // We get `Stdin` here.
    let mut stdout = stdout();

    println!("WORDLE");
    while match game.state() {
        &GameStates::InProgress => true,
        &GameStates::Won => {
            println!("Congratulations!");
            false
        }
        &GameStates::Lost => {
            println!("Answer: {}", game.answer());
            println!("Better luck next time!");
            false
        }
    } {
        let mut buffer = String::new();
        stdin.read_line(&mut buffer)?;
        match game.take_turn(String::from(buffer.trim())) {
            Ok(_) => {
                stdout
                    .queue(crossterm::cursor::MoveToPreviousLine(1))
                    .unwrap();
                stdout
                    .queue(terminal::Clear(terminal::ClearType::CurrentLine))
                    .unwrap();
                let strings = game.turn_strings().unwrap();
                println!("{}", strings);

                stdout.flush().unwrap();
            }
            Err(err) => {
                println!("{}", err.message);
                println!("Try again.")
            }
        }
    }
    Ok(())
}
