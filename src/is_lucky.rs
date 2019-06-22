#[allow(dead_code)]
pub fn run(n: i32) -> bool {
    isLucky(n)
}

#[allow(non_snake_case)]
fn isLucky(n: i32) -> bool {
    let half = ((n as f64).log10() as i32 + 1) as u32 / 2;
    let mut left = n / 10i32.pow(half);
    let mut right = n - (left * 10i32.pow(half));

    let mut result = 0;

    while left != 0 || right != 0 {
        result += left % 10 - right % 10;
        left /= 10;
        right /= 10;
    }

    result == 0
}
