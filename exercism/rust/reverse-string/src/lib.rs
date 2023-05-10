pub fn reverse(input: &str) -> String {
    let mut output = input.to_string();
    unsafe {
        let vec = output.as_mut_vec();
        vec.reverse();
    }

    output
}
