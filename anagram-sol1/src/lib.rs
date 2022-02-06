use std::collections::HashSet;

const CASE_MASK: u8 = b'a' ^ b'A';
#[inline(always)]
fn char_lowercase(c: char) -> char {
    if c.is_ascii_alphabetic() {
        (c as u8 | CASE_MASK) as char
    } else if c.is_uppercase() {
        c.to_lowercase().next().unwrap()
    } else {
        c
    }
}

#[inline(always)]
fn string_lowercase(s: &str) -> String {
    s.chars().map(|c| char_lowercase(c)).collect()
}

#[inline(always)]
fn checksum(word: &str) -> u8 {
    word.bytes().fold(0u8, |a, b| a.overflowing_add(b).0)
}

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let word = string_lowercase(&word);
    let w_cs = checksum(&word);
    let wlen = word.len();
    possible_anagrams
        .iter()
        .filter(|&pa| {
            let pa = string_lowercase(pa);
            if pa.len() != wlen || w_cs != checksum(&pa) || pa == word {
                return false;
            }
            let mut len: usize = 0;
            pa.chars().all(|c| {
                len += 1;
                len > wlen
                    || word.chars().filter(|&x| x == c).count()
                        == pa.chars().filter(|&x| x == c).count()
            })
        })
        .map(|&x| x)
        .collect()
}




#[cfg(test)]
mod tests {
	use super::*;
	//use resistor_color::{color_to_value, colors, value_to_color_string, ResistorColor};
	fn process_anagram_case(word: &str, inputs: &[&str], expected: &[&str]) {
		let result = super::anagrams_for(word, inputs);

		let expected: HashSet<&str> = expected.iter().cloned().collect();

		assert_eq!(result, expected);
	}

	#[test]
	fn test_no_matches() {
		let word = "diaper";

		let inputs = ["hello", "world", "zombies", "pants"];

		let outputs = vec![];

		process_anagram_case(word, &inputs, &outputs);
	}

	#[test]
	#[ignore]
	fn test_detect_simple_anagram() {
		let word = "ant";

		let inputs = ["tan", "stand", "at"];

		let outputs = vec!["tan"];

		process_anagram_case(word, &inputs, &outputs);
	}

	#[test]
	#[ignore]
	fn test_does_not_confuse_different_duplicates() {
		let word = "galea";

		let inputs = ["eagle"];

		let outputs = vec![];

		process_anagram_case(word, &inputs, &outputs);
	}

	#[test]
	#[ignore]
	fn test_eliminate_anagram_subsets() {
		let word = "good";

		let inputs = ["dog", "goody"];

		let outputs = vec![];

		process_anagram_case(word, &inputs, &outputs);
	}

	#[test]
	#[ignore]
	fn test_detect_anagram() {
		let word = "listen";

		let inputs = ["enlists", "google", "inlets", "banana"];

		let outputs = vec!["inlets"];

		process_anagram_case(word, &inputs, &outputs);
	}

	#[test]
	#[ignore]
	fn test_multiple_anagrams() {
		let word = "allergy";

		let inputs = [
		    "gallery",
		    "ballerina",
		    "regally",
		    "clergy",
		    "largely",
		    "leading",
		];

		let outputs = vec!["gallery", "regally", "largely"];

		process_anagram_case(word, &inputs, &outputs);
	}

	#[test]
	#[ignore]
	fn test_case_insensitive_anagrams() {
		let word = "Orchestra";

		let inputs = ["cashregister", "Carthorse", "radishes"];

		let outputs = vec!["Carthorse"];

		process_anagram_case(word, &inputs, &outputs);
	}

	#[test]
	#[ignore]
	fn test_unicode_anagrams() {
		let word = "ΑΒΓ";

		// These words don't make sense, they're just greek letters cobbled together.
		let inputs = ["ΒΓΑ", "ΒΓΔ", "γβα"];

		let outputs = vec!["ΒΓΑ", "γβα"];

		process_anagram_case(word, &inputs, &outputs);
	}

	#[test]
	#[ignore]
	fn test_misleading_unicode_anagrams() {
		// Despite what a human might think these words contain different letters, the input uses Greek
		// A and B while the list of potential anagrams uses Latin A and B.
		let word = "ΑΒΓ";

		let inputs = ["ABΓ"];

		let outputs = vec![];

		process_anagram_case(word, &inputs, &outputs);
	}

	#[test]
	#[ignore]
	fn test_does_not_detect_a_word_as_its_own_anagram() {
		let word = "banana";

		let inputs = ["banana"];

		let outputs = vec![];

		process_anagram_case(word, &inputs, &outputs);
	}

	#[test]
	#[ignore]
	fn test_does_not_detect_a_differently_cased_word_as_its_own_anagram() {
		let word = "banana";

		let inputs = ["bAnana"];

		let outputs = vec![];

		process_anagram_case(word, &inputs, &outputs);
	}

	#[test]
	#[ignore]
	fn test_does_not_detect_a_differently_cased_unicode_word_as_its_own_anagram() {
		let word = "ΑΒΓ";

		let inputs = ["ΑΒγ"];

		let outputs = vec![];

		process_anagram_case(word, &inputs, &outputs);
	}

	#[test]
	#[ignore]
	fn test_same_bytes_different_chars() {
		let word = "a⬂"; // 61 E2 AC 82

		let inputs = ["€a"]; // E2 82 AC 61

		let outputs = vec![];

		process_anagram_case(word, &inputs, &outputs);
	}

	#[test]
	#[ignore]
	fn test_different_words_but_same_ascii_sum() {
		let word = "bc";

		let inputs = ["ad"];

		let outputs = vec![];

		process_anagram_case(word, &inputs, &outputs);
	}

}
