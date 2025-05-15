use std::collections::HashMap;

pub fn sort(vector: Vec<i64>) {
    let mut map = HashMap::new();
    for i in vector {
        let count = map.entry(i).or_insert(0);
        *count += 1
    }

    println!("{:?}", map);
}
