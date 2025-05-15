use hashmap_exercise::sort::sort;
use hashmap_exercise::struct_list::List;
use hashmap_exercise::vector_operations::integer_list;
fn main() {
    let mut integer_list = List {
        i_list: integer_list(1, 15),
        list_len: integer_list(1, 15).len() as f64,
        accumulator: 0,
    };

    for element in &integer_list.i_list {
        integer_list.accumulator += element;
    }

    println!(
        "media: {}, mediana: {}",
        integer_list.media(),
        integer_list.mediana(),
    );

    sort(vec![1, 1, 23, 4, 1, 2, 2, 1, 43, 1, 2, 4, 5, 4, 5]);
}
