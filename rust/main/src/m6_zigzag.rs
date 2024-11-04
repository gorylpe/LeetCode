use std::time::SystemTime;

struct Solution;

impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        if num_rows == 1 {
            return s;
        }
        let sb = s.as_bytes();
        let len = sb.len() as i32;
        if len <= num_rows {
            return s;
        }

        let mut r = Vec::new();

        let last_row = num_rows - 1;
        let group_interval = num_rows - 1;
        let group_interval_2 = group_interval * 2;
        let groups = (len - 1) / group_interval + 1; // Ceiling from division to groups

        // First row
        let start = 0;
        let mut row_start = 0;
        for _ in (0..groups).step_by(2) {
            let i = row_start + start;
            r.push(sb[i as usize]);
            row_start += group_interval_2;
        }

        // Middle rows
        for start in 1..last_row {
            let mut row_start = 0;
            let mut start_switch = start;
            for _ in 0..groups {
                let i = row_start + start_switch;
                if i < len {
                    r.push(sb[i as usize]);
                }
                start_switch = last_row - start_switch;
                row_start += group_interval;
            }
        }

        // Last row
        let start = last_row;
        let mut row_start = 0;
        for _ in (0..groups).step_by(2) {
            let i = row_start + start;
            if i < len {
                r.push(sb[i as usize]);
            }
            row_start += group_interval_2;
        }
        String::from_utf8(r).unwrap()
    }
}

pub fn test() {
    let start = SystemTime::now();
    assert_eq!(Solution::convert("ABCDEFGHIJKL".to_string(), 3).as_str(), "AEIBDFHJLCGK");
    assert_eq!(Solution::convert("ABCDEFGHIJK".to_string(), 3).as_str(), "AEIBDFHJCGK");
    assert_eq!(Solution::convert("ABCDEFGHIJ".to_string(), 3).as_str(), "AEIBDFHJCG");
    assert_eq!(Solution::convert("ABCDEFGHI".to_string(), 3).as_str(), "AEIBDFHCG");
    assert_eq!(Solution::convert("ABCDEFGH".to_string(), 3).as_str(), "AEBDFHCG");
    assert_eq!(Solution::convert("ABCDEFG".to_string(), 3).as_str(), "AEBDFCG");
    assert_eq!(Solution::convert("ABCDEF".to_string(), 3).as_str(), "AEBDFC");
    assert_eq!(Solution::convert("ABCDE".to_string(), 3).as_str(), "AEBDC");
    assert_eq!(Solution::convert("ABCDEFGHIJKLMNOP".to_string(), 4).as_str(), "AGMBFHLNCEIKODJP");
    assert_eq!(Solution::convert("ABCDEFGHIJKLMNO".to_string(), 4).as_str(), "AGMBFHLNCEIKODJ");
    assert_eq!(Solution::convert("ABCDEFGHIJKLMN".to_string(), 4).as_str(), "AGMBFHLNCEIKDJ");
    assert_eq!(Solution::convert("ABCDEFGHIJKLM".to_string(), 4).as_str(), "AGMBFHLCEIKDJ");
    assert_eq!(Solution::convert("ABCDEFGHIJKL".to_string(), 4).as_str(), "AGBFHLCEIKDJ");
    assert_eq!(Solution::convert("ABCDEFGHIJK".to_string(), 4).as_str(), "AGBFHCEIKDJ");
    assert_eq!(Solution::convert("ABCDEFGHIJ".to_string(), 4).as_str(), "AGBFHCEIDJ");
    assert_eq!(Solution::convert("ABCDEFGHI".to_string(), 4).as_str(), "AGBFHCEID");
    assert_eq!(Solution::convert("A".to_string(), 1).as_str(), "A");
    assert_eq!(Solution::convert("A".to_string(), 2).as_str(), "A");
    assert_eq!(Solution::convert("AB".to_string(), 1).as_str(), "AB");
    assert_eq!(Solution::convert("AB".to_string(), 2).as_str(), "AB");
    assert_eq!(Solution::convert("PAYPALISHIRING".to_string(), 3).as_str(), "PAHNAPLSIIGYIR");
    assert_eq!(Solution::convert("PAYPALISHIRING".to_string(), 4).as_str(), "PINALSIGYAHRPI");
    println!("{:?}", start.elapsed().unwrap())
}