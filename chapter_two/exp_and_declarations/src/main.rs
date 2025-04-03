fn main() {
    //  Declarações são instruções que executam alguma ação e não retornam um resultado.
    //Expressões avaliam algo e retornam um resultado elas compõem a maior parte do código que você escreverá em Rust

    // declarações terminam com ;
    // expressões não

    // Declaração ex:
    let x = 43;

    // expressões podem fazer parte de declarações
    // na declaração acima, 43 é uma expressão que avalia o valor 43
    let y = 2 + 3;
    // aqui 2+3 é uma expressão que avalia o valor 5

    // A chamada de função é uma expressão.
    //  Chamar uma macro é uma expressão.
    // O bloco que vamos usar para criar um novo escopo, {}, é uma expressão
    // exemplo:

    let y = {
        let x = 3;
        x + 1
        // Temos aqui duas declarações e duas expressões
        // O {} é uma expressão que faz parte de uma declaração
        // O 3 é uma expressão que faz parte de uma declaração
        // x + 1 também é uma expressão, e avalia em 4
    };

    println!("{}", y);

    // O bloco acima é uma  que avalia em 4. devemos observar que "x+1" não tem ;,
    //  pois se colocar ; o bloco vira uma declaração e acaba não retornado nada


    
    let number = 32;
    // Uma tupla de "expressões"
    // aqui como só temos a expressão dentro do bloco, poderiamos deixar somente a exp sem o bloco
    let my_tup = ({ number < 10 }, { number > 10 });

    let my_exp = {
        if my_tup.1 {
            "menor"
        } else {
            "maior"
        }
    };

    println!("my_exp: {}", my_exp);
}
