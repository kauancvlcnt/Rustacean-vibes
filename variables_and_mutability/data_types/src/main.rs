fn main() {
    let age = "50";
    // Sombreada e tranfomada
    // Quando fazemos parse, devemos declarar para qual tipo de dado queremos tranformar
    let age:u32 = match age.parse() {
        Ok(num) => num, //Quando Ok, "num" o número que foi feito parse é retornado
        Err(_) => 0,

    };

    // Muito parecido com o if else, se Ok retorna o num else retorna 0

    println!("{age}");

    println!("-------------Tipos escalares-------------");

    // Tipos escalares representam um valor único. Rust tem 4 tipos escalares primários
    //  inteiros,
    //  números de ponto flutuante,
    //  booleanos,
    //  caracteres
    
    // inteiros com sinal começam com i, inteiros sem sinal começam com u
    // Signed      Unsigned
    // i8             u8
    // i16            u16
    // i32            u32
    // i64            u64
    // isize          usize

    //Signed pode ser negativo
    //Unsigned somente positivo

    // Cada variante com sinal pode armazendar de -(2^n-¹) até 2^n-¹ -1
    // ex: um i8 pode armazendar de -128 até 127 -(2⁷) até 2⁷-1 que dá 255 possibilidades, ou seja 8 bits

}
