// Rust é uma linguagem baseada em expressão

fn main() {
    println!("Função main");

    let resultado = calcula_dois_num(8,4);

    println!("resultado: {resultado}");

}

// Rust não se importa onde você definiu suas função.
// Essa função também poderia ser definida antes da main
// Os valores que a função recebe nos parâmetros é chamado de argumento.
// Devemos definir o tipo do parâmetro que queremos receber ex:
fn calcula_dois_num(a: u32,b: u32) -> u32 {
     a+b //expressão que avalia em a+b e vai ser retornado pela função

    //  podemos fazer early return usando a palavra return, porém a função já retorna a ultima expressão implicitamente.
}