#[allow(dead_code)]
pub fn run(strings: Vec<String>) -> Vec<String> {
    allLongestStrings(strings)
}

#[allow(non_snake_case)]
fn allLongestStrings(strings: Vec<String>) -> Vec<String> {
    allLongestStrings_maxBy(strings)
}

#[allow(dead_code)]
#[allow(non_snake_case)]
fn allLongestStrings_fold(strings: Vec<String>) -> Vec<String> {
    let longest = strings
        .iter()
        .fold(0usize, |acc, s| if s.len() > acc { s.len() } else { acc });

    strings.into_iter().filter(|s| s.len() == longest).collect()
}

#[allow(dead_code)]
#[allow(non_snake_case)]
fn allLongestStrings_maxBy(strings: Vec<String>) -> Vec<String> {
    let longest = match strings.iter().max_by(|s1, s2| s1.len().cmp(&s2.len())) {
        Some(x) => x.len(),
        None => 0,
    };

    strings.into_iter().filter(|s| s.len() == longest).collect()
}
