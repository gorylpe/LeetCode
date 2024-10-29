use std::time::SystemTime;

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
        let mut longest = Longest { start: 0, end: 0, size: 1 };
        let mut array = vec![vec![P::None; s.len()]; s.len()];
        Self::longest_palindrome_internal(s.as_bytes(), 0, s.len() - 1, &mut longest, &mut array);
        s[longest.start..longest.end + 1].to_string()
    }

    fn longest_palindrome_internal(s: &[u8], a: usize, b: usize, longest: &mut Longest, cache: &mut Vec<Vec<P>>) -> bool {
        if a >= b {
            return true;
        }
        if cache[a][b] != P::None {
            return cache[a][b] == P::Palindrome;
        }

        let curr = Self::longest_palindrome_internal(s, a + 1, b - 1, longest, cache) && s[a] == s[b];
        if curr {
            cache[a][b] = P::Palindrome;
            Self::update_longest(a, b, b - a + 1, longest);
            return true;
        } else {
            cache[a][b] = P::NonPalindrome;
        }

        let lr_size = b - a + 1 - 1;
        if lr_size > longest.size {
            Self::longest_palindrome_internal(s, a + 1, b, longest, cache);
        }
        if lr_size > longest.size {
            Self::longest_palindrome_internal(s, a, b - 1, longest, cache);
        }

        curr
    }

    fn update_longest(a: usize, b: usize, size: usize, longest: &mut Longest) {
        if size > longest.size {
            longest.size = size;
            longest.start = a;
            longest.end = b;
        }
    }
}

pub fn test() {
    let start = SystemTime::now();
    let first = Solution::longest_palindrome("babad".to_string());
    println!("{}", first);
    assert!(first == "bab" || first == "aba");
    assert_eq!(Solution::longest_palindrome("cbbd".to_string()), "bb");
    let third = Solution::longest_palindrome("abbcccbbbcaaccbababcbcabca".to_string());
    println!("{}", third);
    println!("{:?}", start.elapsed().unwrap())
}