// https://leetcode.com/problems/longest-substring-without-repeating-characters/

use std::collections::HashSet;

/// First of all we can't do better than O(N) where N is the length of the string
/// because we have to at least check the whole string.
/// The idea of this algorithm is to check the string char by char, adding each
/// non-repeating char to an hashset so at the next iteration we can see if that
/// current char is already present in the current substring, whose chars are keys
/// of the hashset.
/// If it is already present, we check if the substring length improves upon the best
/// solution found so far and if it does we update it.
/// Then, we clear the hashset and add the current char to it.
/// If it's not present we simply add it to the hashset.
/// The return value is the length of the best (longest) substring candidate.
pub fn length_of_longest_substring(s: String) -> i32 {
    let mut hs = HashSet::new();
    let mut best = 0;

    for i in s.chars() {
        if hs.contains(&i) {
            if hs.len() > best {
                best = hs.len();
            }
            hs.clear();
        }
        hs.insert(i);
    }
    best as i32
}

#[cfg(test)]
mod tests {
    use super::length_of_longest_substring;

    #[test]
    fn given_example_1() {
        let input = String::from("abcabcbb");
        let output = 3;
        assert_eq!(length_of_longest_substring(input), output);
    }

    #[test]
    fn given_example_2() {
        let input = String::from("bbbbb");
        let output = 1;
        assert_eq!(length_of_longest_substring(input), output);
    }

    #[test]
    fn given_example_3() {
        let input = String::from("pwwkew");
        let output = 3;
        assert_eq!(length_of_longest_substring(input), output);
    }
}