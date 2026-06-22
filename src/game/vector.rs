const POSITIONS: usize = 5;
const LETTERS: usize = 26;
pub const DIMENSIONS: usize = POSITIONS * LETTERS;

#[derive(Debug, Clone)]
pub struct WordVector {
    data: [f64; DIMENSIONS],
}

impl WordVector {
    pub fn from_word(word: &str) -> Self {
        let mut data = [0.0; DIMENSIONS];
        for (pos, byte) in word.bytes().enumerate() {
            if pos >= POSITIONS {
                break;
            }
            let letter = byte.to_ascii_lowercase();
            if letter.is_ascii_lowercase() {
                let letter_idx = (letter - b'a') as usize;
                let idx = pos * LETTERS + letter_idx;
                data[idx] = 1.0;
            }
        }
        WordVector { data }
    }

    pub fn dot(&self, other: &WordVector) -> f64 {
        self.data
            .iter()
            .zip(other.data.iter())
            .map(|(a, b)| a * b)
            .sum()
    }

    pub fn squared_distance(&self, other: &WordVector) -> f64 {
        let matches = self.dot(other);
        let total_norm = (POSITIONS as f64) + (POSITIONS as f64);
        total_norm - 2.0 * matches
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_same_word_distance_zero() {
        let v = WordVector::from_word("hello");
        assert_eq!(v.squared_distance(&v), 0.0);
    }

    #[test]
    fn test_completely_different_words() {
        let a = WordVector::from_word("abcde");
        let b = WordVector::from_word("fghij");
        let dist = a.squared_distance(&b);
        assert!(dist > 0.0);
    }

    #[test]
    fn test_dot_matching_positions() {
        let a = WordVector::from_word("abcde");
        let b = WordVector::from_word("abxyz");
        assert_eq!(a.dot(&b), 2.0);
    }

    #[test]
    fn test_from_word_case_sensitivity() {
        let lower = WordVector::from_word("apple");
        let upper = WordVector::from_word("APPLE");
        assert_eq!(lower.dot(&upper), 5.0);
    }

    #[test]
    fn test_short_word_ignores_missing_positions() {
        let a = WordVector::from_word("ab");
        let b = WordVector::from_word("abcde");
        assert_eq!(a.dot(&b), 2.0);
    }

    #[test]
    fn test_squared_distance_formula() {
        let a = WordVector::from_word("abcde");
        let b = WordVector::from_word("abxyz");
        let dist = a.squared_distance(&b);
        assert!((dist - 6.0).abs() < f64::EPSILON);
    }
}
