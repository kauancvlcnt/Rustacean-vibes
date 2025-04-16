// Dados somente da stack: Copy


fn main() {
    let x = 5;
    let y = x; // y é uma cópia de x.
    // Isso funciona e é válido mas x não é invalidado, pois ele é um valor copy

    // Valores copy, são dados armazenados diretamente na stack, eles são dados
    // conhecidos em tempo de compilação por isso são fáceis e rápidos de copiar
    // Valores copy não são invalidados quando fazemos o tipo de exemplo acima
    println!("{x} {y}");

    // Se um tipo possui a trait Copy a variável anterior continua funcionando

    // Tipos Copy
    // Escalares em geral são Copy, inteiros, booleanos, char e ponto floats
    // Tuplas (somente tuplas com tipos escalares) ou seja, tuplas que contém
    // apenas valores  Copy

    
}
