#[allow(dead_code)]
pub fn run(matrix: Vec<Vec<i32>>) -> i32 {
    matrixElementsSum(matrix)
}

#[allow(non_snake_case)]
fn matrixElementsSum(matrix: Vec<Vec<i32>>) -> i32 {
    let mut total = 0;
    let mut zero = vec![false; matrix[0].len()];

    for row in matrix.iter() {
        for (j, val) in row.iter().enumerate() {
            if zero[j] {
                continue;
            } else if *val == 0 {
                zero[j] = true;
            } else {
                total += val;
            }
        }
    }

    total
}
