pub mod results;

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use std::fmt;
use std::time::Instant;

pub struct TestEvent {
    pub time: Instant,
    pub key: KeyEvent,
    pub correct: Option<bool>,
}

impl fmt::Debug for TestEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TestEvent")
            .field("time", &String::from("Instant { ... }"))
            .field("key", &self.key)
            .finish()
    }
}

#[derive(Debug)]
pub struct TestWord {
    pub text: String,
    pub progress: String,
    pub events: Vec<TestEvent>,
}

impl From<String> for TestWord {
    fn from(string: String) -> Self {
        TestWord {
            text: string,
            progress: String::new(),
            events: Vec::new(),
        }
    }
}

#[derive(Debug)]
pub struct Test {
    pub words: Vec<TestWord>,
    pub current_word: usize,
    pub complete: bool,
}

impl Test {
    pub fn new(words: Vec<String>) -> Self {
        Self {
            words: words.into_iter().map(TestWord::from).collect(),
            current_word: 0,
            complete: false,
        }
    }

    pub fn handle_key(&mut self, key: KeyEvent) {
        if key.modifiers.contains(KeyModifiers::CONTROL) && key.code == KeyCode::Char('h') {
            if self.words[self.current_word].progress.is_empty() {
                self.last_word();
            }

            let word = &mut self.words[self.current_word];

            word.events.push(TestEvent {
                time: Instant::now(),
                correct: None,
                key,
            });
            word.progress.clear();
            return;
        }

        let word = &mut self.words[self.current_word];
        match key.code {
            KeyCode::Char(' ') | KeyCode::Enter => {
                if !word.progress.is_empty() {
                    if word.text.chars().nth(word.progress.len()) != Some(' ') {
                        word.events.push(TestEvent {
                            time: Instant::now(),
                            correct: Some(word.text == word.progress),
                            key,
                        });
                        self.next_word();
                    } else {
                        word.progress.push(' ');
                        word.events.push(TestEvent {
                            time: Instant::now(),
                            correct: Some(true),
                            key,
                        })
                    }
                }
            }
            KeyCode::Backspace => {
                if word.progress.is_empty() {
                    self.last_word();
                } else {
                    word.events.push(TestEvent {
                        time: Instant::now(),
                        correct: Some(!word.text.starts_with(&word.progress[..])),
                        key,
                    });
                    word.progress.pop();
                }
            }
            KeyCode::Char(c) => {
                word.progress.push(c);
                word.events.push(TestEvent {
                    time: Instant::now(),
                    correct: Some(word.text.starts_with(&word.progress[..])),
                    key,
                });
                if word.progress == word.text && self.current_word == self.words.len() - 1 {
                    self.complete = true;
                    self.current_word = 0;
                }
            }
            _ => {}
        };
    }

    fn last_word(&mut self) {
        if self.current_word != 0 {
            self.current_word -= 1;
        }
    }

    fn next_word(&mut self) {
        if self.current_word == self.words.len() - 1 {
            self.complete = true;
            self.current_word = 0;
        } else {
            self.current_word += 1;
        }
    }
}
