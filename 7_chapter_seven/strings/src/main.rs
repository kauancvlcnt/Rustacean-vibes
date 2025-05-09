fn main() {
    // Strings em Rust são implementadas como uma coleção de bytes mais alguns métodos
    //para fornecer informações úteis e funcionalidade quando esses bytes são
    //interpretados como texto

    //Rust só tem um tipo de string no  núcleo da pŕopria linguagem: str, um slice
    //de string, que geralmente é vista na forma emprestada, &str.

    //Muitas das operações que se pode fazer com vetores, são possiveis fazer
    //com strings
    let mut ne: String = String::new();

    ne.push_str("oi");

    println!("{ne}")
}
