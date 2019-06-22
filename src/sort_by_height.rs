#[allow(dead_code)]
pub fn run(vec: Vec<i32>) -> Vec<i32> {
    sortByHeight(vec)
}

#[allow(non_snake_case)]
fn sortByHeight(vals: Vec<i32>) -> Vec<i32> {
    let mut nums = vec![];
    let mut result: Vec<i32> = vec![];

    for val in vals.iter() {
        if *val != -1 {
            nums.push(*val)
        }
    }

    nums.sort();

    let mut i = 0;
    for val in vals.iter() {
        if *val != -1 {
            result.push(nums[i]);
            i += 1;
        } else {
            result.push(-1);
        }
    }

    result
}
