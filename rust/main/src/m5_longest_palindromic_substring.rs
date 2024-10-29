struct Solution;

#[derive(PartialEq, Eq, Clone, Debug)]
enum P {
    None,
    Palindrome,
    NonPalindrome,
}

struct Longest {
    start: usize,
    end: usize,
    size: usize,
}

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let mut longest = Longest { start: 0, end: 0, size: 0 };
        let mut array = vec![vec![P::None; s.len()]; s.len()];
        Self::longest_palindrome_internal(s.as_bytes(), 0, s.len() - 1, &mut longest, &mut array);
        s[longest.start..longest.end + 1].to_string()
    }

    fn longest_palindrome_internal(s: &[u8], a: usize, b: usize, longest: &mut Longest, array: &mut Vec<Vec<P>>) -> bool {
        if a == b || a == b - 1 {
            if s[a] == s[b] {
                Self::update_longest(a, b, longest);
                return true;
            }
            return false;
        }

        Self::longest_palindrome_internal(s, a, b - 1, longest, array);
        Self::longest_palindrome_internal(s, a + 1, b, longest, array);
        if Self::longest_palindrome_internal(s, a + 1, b - 1, longest, array) && s[a] == s[b] {
            Self::update_longest(a, b, longest);
            return true;
        }
        false
    }

    fn update_longest(a: usize, b: usize, longest: &mut Longest) {
        let ssize = b - a + 1;
        if ssize > longest.size {
            longest.size = ssize;
            longest.start = a;
            longest.end = b;
        }
    }
}

pub fn test() {
    let first = Solution::longest_palindrome("babad".to_string());
    println!("{}", first);
    assert!(first == "bab" || first == "aba");
    assert_eq!(Solution::longest_palindrome("cbbd".to_string()), "bb");
    let third = Solution::longest_palindrome("abbcccbbbcaaccbababcbcabca".to_string());
    println!("{}", third);
}