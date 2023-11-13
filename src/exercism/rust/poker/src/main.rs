use poker::*;
use std::collections::HashSet;

fn main() {
    test(&["4S 5H 6C 8D KH", "2S 4H 6S 4D JH"], &["2S 4H 6S 4D JH"]);
}

fn hs_from<'a>(input: &[&'a str]) -> HashSet<&'a str> {
    let mut hs = HashSet::new();
    for item in input.iter() {
        hs.insert(*item);
    }
    hs
}

/// Test that the expected output is produced from the given input
/// using the `winning_hands` function.
///
/// Note that the output can be in any order. Here, we use a HashSet to
/// abstract away the order of outputs.
fn test(input: &[&str], expected: &[&str]) {
    assert_eq!(hs_from(&winning_hands(input)), hs_from(expected));
    println!("Success");
}
