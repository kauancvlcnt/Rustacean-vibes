// Referências e empréstimos
fn main() {
    let s1 = String::from("texto");
    // esses & são referências
    // o oposto de referenciar, é derreferenciar, feito pelo operador *
    let tamanho = calcula_tamanho(&s1); //s1 foi passado sem tranferir a posse para a função
                                        // calcula_tamanho, foi um "empréstimo"
    // s1 ainda funcionará aqui, pois o valor move foi passado apenas por referência
    println!("O tamanho de {} é {}", s1, tamanho);
}

fn calcula_tamanho(s: &String) -> usize {
    //s recebe um ponteiro que aponta para s1, sem tomar posse do valor
    // s -> s1 -> memória (s aponta para s1, s1 aponta para a região de memória onde está o dado)
    s.len()
}
