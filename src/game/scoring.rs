use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LetterState {
    Green,
    Yellow,
    Grey,
}

pub const WORD_LENGTH: usize = 5;

pub fn score_guess(guess: &str, answer: &str) -> [LetterState; WORD_LENGTH] {
    let guess = guess.as_bytes();
    let answer = answer.as_bytes();
    let mut result = [LetterState::Grey; WORD_LENGTH];
    let mut used = [false; WORD_LENGTH];

    for i in 0..WORD_LENGTH {
        if guess[i] == answer[i] {
            result[i] = LetterState::Green;
            used[i] = true;
        }
    }

    for i in 0..WORD_LENGTH {
        if result[i] == LetterState::Green {
            continue;
        }
        for j in 0..WORD_LENGTH {
            if !used[j] && guess[i] == answer[j] {
                result[i] = LetterState::Yellow;
                used[j] = true;
                break;
            }
        }
    }

    result
}

pub fn count_yellows(guess: &str, answer: &str) -> usize {
    let guess = guess.as_bytes();
    let answer = answer.as_bytes();
    let mut used = [false; WORD_LENGTH];
    let mut yellows = 0;

    for i in 0..WORD_LENGTH {
        if guess[i] == answer[i] {
            used[i] = true;
        }
    }

    for i in 0..WORD_LENGTH {
        if guess[i] == answer[i] {
            continue;
        }
        for j in 0..WORD_LENGTH {
            if !used[j] && guess[i] == answer[j] {
                yellows += 1;
                used[j] = true;
                break;
            }
        }
    }

    yellows
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_green() {
        let result = score_guess("hello", "hello");
        assert_eq!(result, [LetterState::Green; 5]);
    }

    #[test]
    fn test_all_grey() {
        let result = score_guess("abcde", "fghij");
        assert_eq!(result, [LetterState::Grey; 5]);
    }

    #[test]
    fn test_yellow() {
        let result = score_guess("heart", "earth");
        assert_eq!(result[0], LetterState::Yellow);
    }

    #[test]
    fn test_duplicate_letters_only_one_yellow() {
        let result = score_guess("aaabc", "daeaf");
        assert_eq!(result[0], LetterState::Yellow);
        assert_eq!(result[1], LetterState::Green);
    }

    #[test]
    fn test_count_yellows_basic() {
        let n = count_yellows("abcde", "xbyez");
        assert_eq!(n, 1);
    }

    #[test]
    fn test_count_yellows_none() {
        let n = count_yellows("abcde", "fghij");
        assert_eq!(n, 0);
    }
}
