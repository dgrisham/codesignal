#[allow(dead_code)]
pub fn run(input: String) -> String {
    reverseInParentheses(input)
}

#[allow(non_snake_case)]
fn reverseInParentheses(input: String) -> String {
    let mut result = String::with_capacity(input.len());
    let mut stack: Vec<char> = vec![];
    let mut parens_level: i32 = 0;

    for c in input.chars() {
        if c == '(' {
            parens_level += 1;
            result.push('(');
        } else if c == ')' {
            parens_level -= 1;
            while !stack.is_empty() {
                match stack.pop() {
                    Some(c) => result.push(c),
                    None => (),
                };
            }
            result.push(')');
        } else if parens_level.is_even() {
            // TODO
            result.push(c);
        } else {
            stack.push(c);
        }
    }

    result
}
