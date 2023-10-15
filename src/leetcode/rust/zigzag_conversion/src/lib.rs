pub struct Solution {}

impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        let rows = num_rows as usize;
        if rows == 1 || rows >= s.len() {
            return s;
        }

        let mut result = String::new();

        let added_index = rows * 2 - 1 - 1;
        let input = s.chars().collect::<Vec<char>>();

        for i in 0..rows {
            // First position
            result.push(input[i]);

            // Center and next position 
            let mut count = 1;
            loop {
                if i != 0 && i != (rows - 1) {
                    let center_index = i + (added_index * count) - (i * 2);
                    if center_index >= input.len() {
                        break;
                    }
                    result.push(input[center_index]);
                }
    
                let next_index = i + (added_index * count);
                if next_index >= input.len() {
                    break;
                }
                
                result.push(input[next_index]);

                count = count + 1;
            }
        }

        result
    }
}
