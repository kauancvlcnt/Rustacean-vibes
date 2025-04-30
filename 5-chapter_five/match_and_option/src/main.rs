fn main() {
    //Some é um Option
    let cinco = Some(5);
    let seis = mais_um(cinco);
    //None também é um Option
    let nenhum = mais_um(None);

    println!("{:#?}, {:#?}", seis, nenhum);
}

fn mais_um(x:Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1), //i recebe  o 5 que foi passado no Some()
        _ => Some(1), //esse _ casa qualquer valor
    }

    // None e Some são options
 
}
