fn main() {
    let age = "50";
    // Sombreada e tranfomada
    // Quando fazemos parse, devemos declarar para qual tipo de dado queremos tranformar
    let age: u32 = match age.parse() {
        Ok(num) => num, //Quando Ok, "num", o número que foi feito parse é retornado
        Err(_) => 0,    //quando Err retorna 0.  "_" é um pega tudo
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

    let x: u8 = 50;
    //Signed pode ser negativo
    //Unsigned somente positivo

    // Cada variante com sinal pode armazendar de -(2^n-¹) até 2^n-¹ -1
    // ex: um i8 pode armazendar de -128 até 127 -(2⁷) até 2⁷-1 que dá 255 possibilidades, ou seja: 8 bits

    // Ponto flutuante
    let x = 2.0; //f64 double precision
    let y: f32 = 3.0; //f32 simple precision

    // operações numéricas
    // adição
    let soma = 5 + 10;
    //subtração
    let diferenca = 95.5 - 32.0;
    //multiplicação
    let produto = 4 * 30;
    // divisão
    let quociente = 56.7 / 32.4;
    //resto
    let resto = 23 % 5;

    // booleanos
    let t = true;
    let f = false;

    // Tipo char
    // O tipo char armazena apenas um caractere unicode, pode ser qualquer caractere. contato que seja um só
    let c = 'z';
    let z = 'f';

    println!("---------------Tipos compostos-----------------");

    //                                       Tuplaero
    //Uma tupla é de modo geral uma forma de agrupar um certo número de valores com uma variável do tipo composto
    // os elementos da tupla não necessitam  serem de tipos iguais

    let tup: (i32, i8, u32, f64) = (-500, 100, 800, 4.0); //primeiro índice é zero
                                                          // o rust também consegue inferir os tipos da tupla automaticamente, veja abaixo depois de sombrear
    let tup = tup;

    // Para desestruturar a tupla fazemos:
    let (w, x, y, z) = tup;
    // cada valor da tupla agora é representado como uma variável

    // Podemos também acessar o elemento da tupla com um "."
    println!("{}", tup.0);

    let oitocentos = tup.2;

    //                              Matriz

    // Outra maneira de ter uma coleção de vários valores é uma matriz.
    // Diferentemente de uma tupla, todos os elementos de uma matriz
    // devem ser do mesmo tipo e ter tamanho fixo não podendo aumentar e nem diminuir
    // Matrizes são úteis quando você deseja que seus dados sejam alocados na stack ao invés da heap
    //   ou quando você quer garantir que sempre terá um número fixo de elementos
    // Uma matriz é um pedaço da memória alocada na stack.

    // Um vetor é semelhante a uma matriz, porém pode aumentar e diminuir

    let matriz: [u32; 4] = [34, 43, 32, 32];
    //Também podemos desestruturar a matriz
    let [a, b, c, d] = matriz;

    let primeiro = matriz[0];
    let segundo = matriz[1];
    println!("{}  {}", primeiro, segundo);
    // Se tentar acessar um índice da matriz que não existe,
    // o Rust impede o acesso de memória inválida entrando em panico
    // Esse é o primeiro exemplo dos princípios de sugurança do Rust em ação
    
}
