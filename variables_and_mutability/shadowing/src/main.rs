fn main() {
    //  Shadowing você pode declarar uma nova variável com o mesmo nome
    //  de uma variável anterior, e a nova variável sombreia a variável anterior

    // O sombreamento é declarar uma outra variável com o mesmo nome, é interessante quando
    // é necessário fazer um parse de algum valor ou quer fazer uma atualização do dado anterior

    //Exemplo

    let x = 1;
    println!("Variável: {x}");
    let x = x + 30;
    println!("Variável sombreada 1: {x}");
    let x = 55;
    println!("Variável sombreada 2:{x}");

    // Usando shadowing podemos tranformar o tipo da variável ao contrário da declaração de variável mutável
    // let mut testando = 12;
    // testando = "teste"; aqui ocorreria um erro de tipo, pois ela não pode assumir outro tipo além do primeiro
    // declarado

    let x = "bom dia";

    println!("Variável sombreada 3, tipo string: {x}");
}
