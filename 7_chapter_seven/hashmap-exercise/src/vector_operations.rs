pub fn integer_list(initial_range: i64, final_range: i64) -> Vec<i64> {
    let mut int_list: Vec<i64> = Vec::new();
    for int in initial_range..final_range + 1 {
        int_list.push(int);
    }
    int_list //list of integers
}
