// Decidir se deve ou não executar algum código, dependendo se uma condição é verdadeira
//  e decidir executar algum código repetidamente enquanto uma condição é verdadeira
fn main() {
    let numero = 7;
    if numero < 5 {
        //Blocos de código associados às condições em expressões if são às vezes chamado de divisões
        println!("verdadeiro");
    } else {
        //else é opcional,  se não houver o programa seguirá para o próximo bloco de código
        println!("condição falsa");
    }

    // Rust não tentará converter tipos não booleanos em tipos booleanos
    // Deve ser explicitado e sempre fornecer if com um booleano como sua condição

    // Se quisermos que o bloco de código if seja executado somente quando um número não é igual a 0,
    //  por exemplo, podemos mudar o if para o seguinte:

    if numero != 0 {
        println!("número era algo diferente de zero");
    }

    // Rust só executará o primeiro corpo da função que for verdadeira, mesmo que o valor seja divisível por todos
    if numero % 4 == 0 {
        println!("Número  divisível por 4");
    } else if numero % 3 == 0 {
        println!("Número divisível por 3");
    } else if numero % 2 == 0 {
        println!("Número divisível por 2");
    } else {
        println!("Número divisível não é divisível por 2 por 3 e nem por 4")
    }

    // Como if é uma expressão, podemos utilizar ele em uma expressão let

    let condicao = false;

    let numero = if condicao {
        // "sim" os valores das expressões devem ser do mesmo tipo, ser um for u32, os dois devem ser
        //  Rust precisa saber em tempo de compilação qual é o tipo da variável numero
        5
    } else {
        6
        // "não"
    };

    println!("{numero}");

    // laços de repetições
    let mut count = 0;

    loop {
        count += 1;
        println!("print novamente");
        if count == 4 {
            break;
        }
    }

    count = 0;

    while count != 5 {
        count += 1;
        println!("{count}");
    }

    // Looping através de uma coleção com for
    let a: [i32; 5] = [10, 20, 30, 40, 50];
    let mut indice = 0;

    while indice < a.len() {
        //len() pega o tamanho da matriz
        println!("{indice}: {}", a[indice]);
        indice += 1;
    }

    // laço for

    for i in a.iter() {
        //for é mais seguro para fazer iterações, pois ele só itera sobre o range da matriz
        println!("usando for i: {}", i);
    }


    // for usando um Range

    for numero in (0..10).rev() {
         //rev() é um reverse para a sequência. também poderia ser feito apenas invertendo o range
        // se não fosse atribuir um método, poderia deixar sem parêntese
        println!("numero do range contagem regressiva: {}", numero);
    }
}
