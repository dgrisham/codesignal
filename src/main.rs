mod all_longest_strings;
mod common_character_count;
mod is_lucky;
mod matrix_elements_sum;
mod reverse_in_parentheses;
mod sort_by_height;

fn main() {
    let result = reverse_in_parentheses::run("yes(foo(yessir)bar)no".to_string());
    println!("result: {}", result);

    // let result = &*sort_by_height::run(vec![-1, 190, 170, 150, -1, -1, 180, 160]);
    // println!("result: {:?}", result)

    // let result = is_lucky::run(134008);
    // println!("result: {}", result)

    // let result = common_character_count::run("abca".to_string(), "xyzbac".to_string());
    // println!("result: {}", result)

    // let matrix = vec![vec![0, 1, 1, 2], vec![0, 5, 0, 0], vec![2, 0, 3, 3]];
    // let matrix = vec![vec![1, 1, 1, 0], vec![0, 5, 0, 1], vec![2, 1, 3, 10]];
    // let result = matrix_elements_sum::run(matrix);
    // println!("result: {}", result);

    // let strings = vec!["aba", "aa", "ad", "vcd", "aba"]
    //     .into_iter()
    //     .map(|s| s.to_string())
    //     .collect();

    // let result = all_longest_strings::run(strings);
    // for s in result.iter() {
    //     println!("s: {}", s);
    // }
}
