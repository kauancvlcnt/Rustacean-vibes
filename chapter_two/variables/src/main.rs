
//Constantes não podem ser resultados de uma expressão
// Constante são sempre imutáveis
// As constantes podem ser definidas apenas para uma expressão constante, ou seja,
// não pode ser o resultado de uma chamada de função ou qualquer outro valor que 
// só poderia ser calculado em tempo de execução

const INFO_CAIXA: u32 = 30_000; //por convenção constantes são declaradas com todas as letras em maiúsculo e sublinhado entre as palavras
// 30_000 é só uma maneira de dixar o número mais legível
fn main() {
    //    variável imutável
    //   let x = 5;
    // não pode sofrer atualização no futuro

    //  variável mutável
    let mut x = 2;
    println!("o valor de x é: {}", x);
    println!("constante {}", INFO_CAIXA);

    x = 6;
    println!("o valor de x agora é {}", x);
    
}
