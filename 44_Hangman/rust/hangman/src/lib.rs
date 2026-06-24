use rand::prelude::*;

#[derive(Debug)]
pub struct Hangman {
    wordlist: Vec<usize>,
    cur_word: Option<usize>, // index into `wordlist`
    guess: Guess,
}

#[derive(Debug, Default)]
pub struct Guess {
    pub letters_guessed: Vec<char>,
    pub solution_word: Vec<char>,
    pub word_so_far: Vec<char>,
    pub wrong_guesses: usize,
    pub total_guesses: usize,
    pub letters_found: usize,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum GuessResult {
    AlreadyGuessed,
    FoundLetter(usize),
    NotPresent,
    FoundWord,
}

impl Default for Hangman {
    fn default() -> Self {
        Self::new()
    }
}

impl Hangman {
    pub fn new() -> Self {
        let mut rng = rand::rng();
        let mut wordlist = (0..WORDLIST.len()).collect::<Vec<_>>();
        wordlist.shuffle(&mut rng);
        let word = WORDLIST[wordlist[0]];
        Self {
            wordlist,
            cur_word: Some(0),
            guess: Guess::new(word),
        }
    }

    pub fn current_state(&self) -> &Guess {
        &self.guess
    }

    pub fn guess_letter(&mut self, letter: char) -> GuessResult {
        if self.cur_word.is_none() {
            return GuessResult::NotPresent;
        }

        if self.guess.letters_guessed.contains(&letter) {
            return GuessResult::AlreadyGuessed;
        }
        self.guess.letters_guessed.push(letter);

        self.guess.total_guesses += 1;

        let mut found = false;
        if self.guess.solution_word.contains(&letter) {
            for (index, l) in self.guess.solution_word.iter().enumerate() {
                if *l == letter {
                    self.guess.word_so_far[index] = letter;
                    self.guess.letters_found += 1;
                    found = true;
                }
            }
        }

        if self.guess.letters_found == self.guess.solution_word.len() {
            self.move_along();
            GuessResult::FoundWord
        } else if found {
            GuessResult::FoundLetter(self.guess.total_guesses)
        } else {
            self.guess.wrong_guesses += 1;
            GuessResult::NotPresent
        }
    }

    pub fn guess_word(&mut self, word: &str) -> bool {
        let Some(word_idx) = self.cur_word else {
            return false;
        };

        if word == WORDLIST[self.wordlist[word_idx]] {
            self.move_along();
            true
        } else {
            false
        }
    }

    pub fn move_along(&mut self) {
        if let Some(idx) = self.cur_word {
            if idx < WORDLIST.len() - 1 {
                self.cur_word = Some(idx + 1);
                self.guess = Guess::new(WORDLIST[self.wordlist[idx + 1]]);
            } else {
                self.cur_word = None;
            }
        }
    }

    pub fn words_left(&self) -> bool {
        self.cur_word.is_some()
    }
}

impl Guess {
    fn new(word: &str) -> Self {
        Self {
            solution_word: word.chars().collect(),
            word_so_far: (0..word.len()).map(|_| '-').collect(),
            ..Default::default()
        }
    }
}

// 700 DATA "GUM","SIN","FOR","CRY","LUG","BYE","FLY"
// 710 DATA "UGLY","EACH","FROM","WORK","TALK","WITH","SELF"
// 720 DATA "PIZZA","THING","FEIGN","FIEND","ELBOW","FAULT","DIRTY"
// 730 DATA "BUDGET","SPIRIT","QUAINT","MAIDEN","ESCORT","PICKAX"
// 740 DATA "EXAMPLE","TENSION","QUININE","KIDNEY","REPLICA","SLEEPER"
// 750 DATA "TRIANGLE","KANGAROO","MAHOGANY","SERGEANT","SEQUENCE"
// 760 DATA "MOUSTACHE","DANGEROUS","SCIENTIST","DIFFERENT","QUIESCENT"
// 770 DATA "MAGISTRATE","ERRONEOUSLY","LOUDSPEAKER","PHYTOTOXIC"
// 780 DATA "MATRIMONIAL","PARASYMPATHOMIMETIC","THIGMOTROPISM"

const WORDLIST: [&str; 50] = [
    "GUM",
    "SIN",
    "FOR",
    "CRY",
    "LUG",
    "BYE",
    "FLY",
    "UGLY",
    "EACH",
    "FROM",
    "WORK",
    "TALK",
    "WITH",
    "SELF",
    "PIZZA",
    "THING",
    "FEIGN",
    "FIEND",
    "ELBOW",
    "FAULT",
    "DIRTY",
    "BUDGET",
    "SPIRIT",
    "QUAINT",
    "MAIDEN",
    "ESCORT",
    "PICKAX",
    "EXAMPLE",
    "TENSION",
    "QUININE",
    "KIDNEY",
    "REPLICA",
    "SLEEPER",
    "TRIANGLE",
    "KANGAROO",
    "MAHOGANY",
    "SERGEANT",
    "SEQUENCE",
    "MOUSTACHE",
    "DANGEROUS",
    "SCIENTIST",
    "DIFFERENT",
    "QUIESCENT",
    "MAGISTRATE",
    "ERRONEOUSLY",
    "LOUDSPEAKER",
    "PHYTOTOXIC",
    "MATRIMONIAL",
    "PARASYMPATHOMIMETIC",
    "THIGMOTROPISM",
];

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_letter() {
        let mut game = Hangman::new();
        assert_eq!(
            game.guess_letter(game.guess.solution_word[0]),
            GuessResult::FoundLetter(1)
        );
    }

    #[test]
    fn test_not_present() {
        let mut game = Hangman::new();
        let choices = game
            .guess
            .solution_word
            .iter()
            .map(|ch| (*ch as u8 - b'A') as usize)
            .collect::<Vec<_>>();
        let mut present = [false; 26];
        for c in choices {
            present[c] = true;
        }

        let mut found = 0;
        for (idx, p) in present.iter().enumerate() {
            if !p {
                assert_eq!(
                    game.guess_letter((idx as u8 + b'A') as char),
                    GuessResult::NotPresent
                );
                found += 1;
            }
        }

        assert_eq!(game.guess.wrong_guesses, found);
    }

    #[test]
    fn test_find_word() {
        let mut game = Hangman::new();
        game.cur_word = Some(game.wordlist.iter().position(|item| *item == 0).unwrap());
        game.guess = Guess::new("GUM");
        game.guess_letter('G');
        game.guess_letter('U');
        assert_eq!(GuessResult::FoundWord, game.guess_letter('M'));
    }

    #[test]
    fn test_already_guessed() {
        let mut game = Hangman::new();
        game.cur_word = Some(game.wordlist.iter().position(|item| *item == 0).unwrap());
        game.guess = Guess::new("GUM");
        game.guess_letter('A');
        assert_eq!(GuessResult::AlreadyGuessed, game.guess_letter('A'));
    }

    #[test]
    fn test_guess_word() {
        let mut game = Hangman::new();
        game.cur_word = Some(game.wordlist.iter().position(|item| *item == 0).unwrap());
        game.guess = Guess::new("GUM");
        println!("{game:?}");
        assert!(game.guess_word("GUM"));
    }

    #[test]
    fn test_finds_all_words() {
        let mut game = Hangman::new();
        for _ in 0..WORDLIST.len() {
            let word = game
                .current_state()
                .solution_word
                .iter()
                .collect::<String>();
            game.guess_word(&word);
        }
        assert!(!game.words_left());
    }
}
