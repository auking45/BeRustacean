use std::collections::HashSet;

pub struct Solution {}

// 1. HashSet에 insert 하면서 중복되는 것이 있는지 비교하고,
// 중복되는 것을 찾았을 때 지금까지의 가장 긴 문자열로 저장
// => HashSet이 삽입 순서를 보장하지 않는 문제 있음
// 2. HashSet에 삽입하면서 중복 여부 체크하고, 시작, 종료 지점 확인 후 문자열 리턴

// * 길이만 리턴한다면 HashSet에 삽입확인해도 될 듯
impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut longest = 0;
        let mut substring: Vec<char> = vec![];

        for c in s.chars() {
            for (i, &v) in substring.iter().enumerate() {
                if v == c {
                    if substring.len() > longest {
                        longest = substring.len();
                    }
                    substring = substring[i + 1..].to_vec();
                    break;
                }
            }
            substring.push(c);
        }

        if substring.len() > longest {
            longest = substring.len();
        }

        longest as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(3, Solution::length_of_longest_substring("abcabcbb".to_string()));
    }

    #[test]
    fn test_2() {
        assert_eq!(1, Solution::length_of_longest_substring("bbbbb".to_string()));
    }

    #[test]
    fn test_3() {
        assert_eq!(3, Solution::length_of_longest_substring("pwwkew".to_string()));
    }    
}
