#[allow(dead_code)]
pub fn run(s1: String, s2: String) -> i32 {
    commonCharacterCount(s1, s2)
}

#[allow(non_snake_case)]
#[allow(dead_code)]
fn commonCharacterCount_hashMap(s1: String, s2: String) -> i32 {
    use std::collections::HashMap;

    let mut h1 = HashMap::new();
    let mut h2 = HashMap::new();

    for c in s1.chars() {
        *h1.entry(c).or_insert(0) += 1;
    }
    for c in s2.chars() {
        *h2.entry(c).or_insert(0) += 1;
    }

    let mut total = 0;
    for (c, n1) in &h1 {
        total += match h2.get(c) {
            Some(n2) => *std::cmp::min(n1, n2),
            None => 0,
        };
    }

    total
}

#[allow(non_snake_case)]
#[allow(dead_code)]
fn commonCharacterCount(s1: String, s2: String) -> i32 {
    commonCharacterCount_hashMap(s1, s2)
}

#[allow(non_snake_case)]
#[allow(dead_code)]
fn commonCharacterCount_iters(s1: String, mut s2: String) -> i32 {
    let mut total = 0;
    for c1 in s1.chars() {
        for i in 0..s2.len() {
            if c1 == s2.chars().nth(i).unwrap() {
                s2.remove(i);
                total += 1;
                break;
            }
        }
    }
    total
}

#[allow(non_snake_case)]
#[allow(dead_code)]
fn commonCharacterCount_fold(s1: String, mut s2: String) -> i32 {
    s1.chars().fold(0i32, |acc, c| match s2.find(c) {
        Some(i) => {
            s2.remove(i);
            acc + 1
        }
        None => acc,
    })
}
