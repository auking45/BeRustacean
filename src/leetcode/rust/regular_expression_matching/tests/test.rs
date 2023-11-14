use regular_expression_matching::*;

#[test]
fn test_1() {
    assert_eq!(false, Solution::is_match("aa".to_string(), "a".to_string()));
}

#[test]
fn test_2() {
    assert_eq!(true, Solution::is_match("aa".to_string(), "a*".to_string()));
}

#[test]
fn test_3() {
    assert_eq!(true, Solution::is_match("ab".to_string(), ".*".to_string()));
}

#[test]
fn test_4() {
    assert_eq!(
        true,
        Solution::is_match("aaa".to_string(), "a*a".to_string())
    );
}

// A &str to String adapter
fn string_matches_pattern(string: &str, pattern: &str) -> bool {
    Solution::is_match(string.to_string(), pattern.to_string())
}

#[test]
fn test_a_asterisk_a() {
    assert!(string_matches_pattern("aaa", "a*a"));
}

#[test]
fn test_many_zero_repeatables() {
    assert!(string_matches_pattern("aaa", "ab*a*c*a"));
}

#[test]
fn test_multiple_repeated_wildcards() {
    assert!(string_matches_pattern("bbbba", ".*a*a"));
}

#[test]
fn test_only_asterisks_and_dots() {
    assert!(string_matches_pattern("ab", ".*.."));
}

#[test]
fn test_many_asterisks_and_dots_and_other_stuff() {
    assert!(string_matches_pattern("ab", ".*..c*"));
}

#[test]
fn extra_test_case_1() {
    assert!(string_matches_pattern(
        "aabcbcbcaccbcaabc",
        ".*a*aa*.*b*.c*.*a*"
    ));
}

#[test]
fn extra_test_case_2() {
    assert!(string_matches_pattern(
        "ccacbcbcccabbab",
        ".c*a*aa*b*.*b*.*"
    ));
}
