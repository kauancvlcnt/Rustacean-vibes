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

    println!("{ne}");

    let str1 = String::from("hello");
    let str2 = String::from(" world");
    // let str3 = str1 + &str2; //str1 foi movida para str3, str3 tem o ownership de str3
    // println!("{}", str3);
    let formatada = format!("{}{}", str1, str2); // melhor que usar + pra concat
    println!("{}", formatada);

    //Em outras linguagens conseguimos acessar caracteres individuais de string
    //através do indice. Em Rust se nós tentamos acessar partes de uma String usando
    // sintaxe de indexação, vamos ter um erro. Ou seja, este código resultará em erro
    // let s1 = String::from("Hello");
    // let h = s1[0];
    //
    // as strings em Rust não suportam a indexação. Assim a próxima pergunta é,
    // por que não? Para responder a isso, temos que conversar um pouco sobre
    // como o Rust armazena strings na memória.

    // Representção interna
    // Uma string é um invólucro sobre um Vec<u8>.
    let len = String::from("hello").len(); // 4
    //len terá o valor 4, o que significa que  que essa string tem 4 bytes
    // de comprimento: cada uma dessas letras leva um byte quando codificado em utf8

    let len = String::from("Здравствуйте").len(); // 24
    //Uma pessoa que pergunte pelo comprimento da string pode dizer que ela
    // ter 12.No entanto, a resposta de Rust é 24. Este é o número de bytes que
    //  é necessário para codificar “Здравствуйте“ em UTF-8, uma vez que cada valor
    //  escalar Unicode leva dois bytes de armazenamento. Assim sendo, um índice nos
    // bytes da string nem sempre se correlaciona com um valor escalar Unicode válido.

    for c in "teste".chars() {
        //o método chars() retornará 5 variáveis do tipo char, e é possível iterar
        //no resultado
        println!("{}", c);
    }

    for c in "teste".bytes() {
        //bytes() retorna os bytes da string
        //o método chars() retornará 5 variáveis do tipo char, e é possível iterar
        //no resultado
        //valores escalares Unicode válidos podem ser constituídos por mais de um byte.
        println!("{}", c);
    }

    // clusters de grafemas pesquisar
}
