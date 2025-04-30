fn main() {
    let my_arr = [1, 2, 33, 3, 3];
    let bind = unite(my_arr);
    println!("{bind}");
}

fn unite(elemnt: [i32; 5]) -> i32 {
    let mut count = 1;
    let mut teste = 0;
    for i in elemnt {
        let result = i;
        teste += result;

        if count <= elemnt.len() {
            count += 1;
        }
    }
    teste
}
