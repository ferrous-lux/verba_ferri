pub use loader::Dictionary;

mod loader {
    use crate::game::scoring::count_yellows;
    use crate::game::vector::WordVector;

    pub struct Dictionary {
        words: Vec<String>,
        vectors: Vec<WordVector>,
    }

    impl Dictionary {
        #[allow(clippy::new_without_default)]
        pub fn new() -> Self {
            let words: Vec<String> = include_str!("words.txt")
                .lines()
                .map(|s| s.trim().to_lowercase())
                .filter(|s| s.len() == 5)
                .collect();

            let vectors: Vec<WordVector> = words.iter().map(|w| WordVector::from_word(w)).collect();

            Dictionary { words, vectors }
        }

        pub fn words(&self) -> &[String] {
            &self.words
        }

        pub fn len(&self) -> usize {
            self.words.len()
        }

        pub fn is_empty(&self) -> bool {
            self.words.is_empty()
        }

        pub fn nearest_match(&self, guess: &str) -> (String, WordVector) {
            let guess_vec = WordVector::from_word(guess);
            let mut best_idx = 0;
            let mut best_greens = -1i32;
            let mut best_yellows = 0usize;

            for (i, vec) in self.vectors.iter().enumerate() {
                let greens = guess_vec.dot(vec).round() as i32;

                if greens > best_greens {
                    best_greens = greens;
                    best_yellows = count_yellows(guess, &self.words[i]);
                    best_idx = i;
                } else if greens == best_greens {
                    let yellows = count_yellows(guess, &self.words[i]);
                    if yellows > best_yellows {
                        best_yellows = yellows;
                        best_idx = i;
                    }
                }
            }

            (self.words[best_idx].clone(), self.vectors[best_idx].clone())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dictionary_loads_words() {
        let dict = loader::Dictionary::new();
        assert!(!dict.is_empty());
        assert!(dict.len() > 100);
    }

    #[test]
    fn test_all_words_are_five_letters() {
        let dict = loader::Dictionary::new();
        for word in dict.words() {
            assert_eq!(word.len(), 5, "Word '{}' is not 5 letters", word);
        }
    }

    #[test]
    fn test_all_words_are_lowercase() {
        let dict = loader::Dictionary::new();
        for word in dict.words() {
            assert_eq!(
                word.to_lowercase(),
                *word,
                "Word '{}' is not lowercase",
                word
            );
        }
    }

    #[test]
    fn test_nearest_match_finds_exact_word() {
        let dict = loader::Dictionary::new();
        let (match_word, _) = dict.nearest_match("hello");
        assert!(!match_word.is_empty());
    }

    #[test]
    fn test_nearest_match_greens_ordered_correctly() {
        let dict = loader::Dictionary::new();
        let (match_word, _) = dict.nearest_match("abcde");
        assert_eq!(match_word.len(), 5);
    }
}
