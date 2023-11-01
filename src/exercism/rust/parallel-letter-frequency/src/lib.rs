use std::cmp::min;
use std::collections::HashMap;
use std::thread;

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    let mut answers = HashMap::<char, usize>::new();

    let input = input.concat().to_lowercase();
    if input.is_empty() {
        return answers;
    }

    let workers = min(input.len(), worker_count);
    let mut work_len = (input.len() / workers).max(1);
    if input.len() % workers != 0 {
        work_len += 1;
    }

    let mut churn = input.chars();
    let mut handles = Vec::with_capacity(workers);
    for _ in 0..workers {
        let chunk = churn.by_ref().take(work_len).collect::<String>();
        let thread = thread::spawn(move || {
            let mut answer = HashMap::<char, usize>::new();
            chunk.chars().for_each(|c| {
                if c.is_alphabetic() {
                    answer.entry(c).and_modify(|v| *v += 1).or_insert(1);
                }
            });

            answer
        });

        handles.push(thread);
    }

    for handle in handles {
        let answer = handle.join().unwrap();
        for (key, val) in answer.iter() {
            answers
                .entry(*key)
                .and_modify(|v| *v += val)
                .or_insert(*val);
        }
    }

    answers
}
