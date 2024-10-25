use std::collections::hash_map::Entry;
use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut m = HashMap::new();
        let ss = s.as_bytes();

        let mut longest = 0;
        let mut current = 0;

        let mut back_i = 0;
        for c in ss {
            match m.entry(c) {
                Entry::Occupied(_) => {
                    if current > longest {
                        longest = current;
                    }
                    loop {
                        let cb = &ss[back_i];
                        back_i += 1;
                        if cb == c {
                            break;
                        } else {
                            current -= 1;
                            m.remove(cb);
                        }
                    }
                }
                Entry::Vacant(v) => {
                    v.insert(true);
                    current += 1;
                }
            }
        }
        if current > longest {
            longest = current;
        }
        longest
    }
}

pub fn test() {
    assert_eq!(Solution::length_of_longest_substring("abcabcbb".to_string()), 3);
    assert_eq!(Solution::length_of_longest_substring("bbbbb".to_string()), 1);
    assert_eq!(Solution::length_of_longest_substring("pwwkew".to_string()), 3);
    assert_eq!(Solution::length_of_longest_substring(" ".to_string()), 1);
    assert_eq!(Solution::length_of_longest_substring(" ,aa .!".to_string()), 4);
    assert_eq!(Solution::length_of_longest_substring("dvdf".to_string()), 3);
}
