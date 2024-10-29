use std::cmp::min;
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
        // let longest = Self::longest_palindrome_dynamic_start(s.as_bytes());
        let longest = Self::longest_palindrome_centers_start(s.as_bytes());
        s[longest.start..longest.end + 1].to_string()
    }

    fn longest_palindrome_centers_start(s: &[u8]) -> Longest {
        let mut longest = Longest { start: 0, end: 0, size: 1 };
        if s.len() == 1 {
            return longest;
        }
        let len = s.len();
        let mut i = (len - 1) / 2;
        let mut inc = 1;
        while i >= 0 && i < len {
            let max_pal_left = i;
            let max_pal_right = len - 1 - i;
            let max_pal_odd = min(max_pal_left, max_pal_right) * 2 + 1;
            let max_pal_even = if max_pal_right > 0 { min(max_pal_left, max_pal_right - 1) * 2 + 2 } else { 0 };

            if max_pal_odd > longest.size {
                Self::longest_palindrome_centers(s, i, i, &mut longest);
            }
            if max_pal_even > longest.size {
                Self::longest_palindrome_centers(s, i, i + 1, &mut longest);
            }
            if max_pal_odd < longest.size && max_pal_even < longest.size {
                break;
            }

            if i + 1 < s.len() {}

            if inc % 2 == 1 {
                i += inc;
            } else {
                i -= inc;
            }
            inc += 1;
        }
        longest
    }

    fn longest_palindrome_centers(s: &[u8], a: usize, b: usize, longest: &mut Longest) {
        let mut a = a;
        let mut b = b;
        loop {
            if s[a] != s[b] {
                a += 1;
                b -= 1;
                break;
            }
            if a == 0 || b == s.len() - 1 {
                break;
            }
            a -= 1;
            b += 1;
        }
        Self::update_longest(a, b, b + 1 - a, longest);
    }

    fn update_longest(a: usize, b: usize, size: usize, longest: &mut Longest) {
        if size > longest.size {
            longest.size = size;
            longest.start = a;
            longest.end = b;
        }
    }

    fn longest_palindrome_dynamic_start(s: &[u8]) -> Longest {
        let mut longest = Longest { start: 0, end: 0, size: 1 };
        let mut cache = vec![vec![P::None; s.len()]; s.len()];
        Self::longest_palindrome_dynamic(s, 0, s.len() - 1, &mut longest, &mut cache);
        longest
    }

    fn longest_palindrome_dynamic(s: &[u8], a: usize, b: usize, longest: &mut Longest, cache: &mut Vec<Vec<P>>) -> bool {
        if a >= b {
            return true;
        }
        if cache[a][b] != P::None {
            return cache[a][b] == P::Palindrome;
        }

        let curr = Self::longest_palindrome_dynamic(s, a + 1, b - 1, longest, cache) && s[a] == s[b];
        if curr {
            cache[a][b] = P::Palindrome;
            Self::update_longest(a, b, b - a + 1, longest);
            return true;
        } else {
            cache[a][b] = P::NonPalindrome;
        }

        let lr_size = b - a + 1 - 1;
        if lr_size > longest.size {
            Self::longest_palindrome_dynamic(s, a + 1, b, longest, cache);
        }
        if lr_size > longest.size {
            Self::longest_palindrome_dynamic(s, a, b - 1, longest, cache);
        }

        curr
    }
}

pub fn test() {
    let start = SystemTime::now();
    let first = Solution::longest_palindrome("babad".to_string());
    println!("{}", first);
    assert!(first == "bab" || first == "aba");
    assert_eq!(Solution::longest_palindrome("cbbd".to_string()), "bb");
    assert_eq!(Solution::longest_palindrome("ccd".to_string()), "cc");
    assert_eq!(Solution::longest_palindrome("abb".to_string()), "bb");
    let third = Solution::longest_palindrome("abbcccbbbcaaccbababcbcabca".to_string());
    println!("{}", third);
    println!("{:?}", start.elapsed().unwrap())
}