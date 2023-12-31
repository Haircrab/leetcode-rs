#![allow(dead_code)]
struct Solution {}

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        // bucket solution
        let mut count_s = vec![0u32; 26];

        s.chars().for_each(|c| {
            count_s[((c as u8) - ('a' as u8)) as usize] += 1;
        });

        t.chars().for_each(|c| {
            let temp = (c as u8) - ('a' as u8);
            if count_s[temp as usize] != 0 {
                count_s[temp as usize] -= 1;
            }
        });

        for i in 0..26 {
            if count_s[i] != 0 {
                return false;
            }
        }

        return true;
    }
}

#[cfg(test)]
#[test]
fn main() {
    assert_eq!(
        Solution::is_anagram(String::from("anagram"), String::from("nagaram")),
        true
    );
    assert_eq!(
        Solution::is_anagram(String::from("rat"), String::from("car")),
        false
    );
}
