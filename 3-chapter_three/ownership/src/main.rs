// Ownership é uma característica do Rust que permite ter garantias de segurança
// de memória sem precisar de um garbage collector.

// A característica central do Rust é ownership

// Algumas linguagens possuem garbage collection (coleta de lixo), que constantemente
// busca segmentos de memória que já não são mais utilizados enquanto o programa executa;
// em outras linguagens, o programador deve alocar e liberar memória de forma explícita.
// Nenhuma característica relacionada ao ownership implica qualquer custo em tempo de execução.

//------------------------------STACK----------------------------------
// A stack armazena valores na ordem em que eles chegam, e os remove na ordem inversa.
// Isto é chamado de last in, first out (último a chegar, primeiro a sair)
// Exemplo:
//Imagine uma stack de pratos: quando você coloca mais pratos, você os põe em cima
// da stack, e quando você precisa de um prato, você pega o que está no topo.

// Dizemos fazer um PUSH na stack quando nos refererimos a inserir dados,
//  e fazer um POP da stack quando nos referimos a remover dados.

// A stack é rápida por conta da forma como ela acessa os dados: ela nunca tem que procurar um lugar
//  para colocar novos dados, ou um lugar de onde obter dados, este lugar é sempre o topo da stack.

// ------------------------------------HEAP-----------------------------------------------------

// é menos organizada: quando colocamos dados na heap, nós pedimos um certo espaço de memória.
// O sistema operacional encontra um espaço vazio em algum lugar na heap que seja grande o suficiente,
//  marca este espaço como em uso, e nos retorna um PONTEIRO, que é o endereço deste local.
//  Este processo é chamado de alocar na heap, e às vezes se abrevia esta frase como apenas "alocação".
// Colocar valores na pilha não é considerado uma alocação. Como o ponteiro tem um tamanho fixo e conhecido,
//  podemos armazená-lo na pilha, mas quando queremos os dados, de fato, temos que seguir o ponteiro.

// Os dados armazenados na stack devem ter tamanhos fixos e conhecidos

// fn main() {
//     // Um escopo é a área dentro de um programa para a qual um item é válido

//     // s ainda não é válida aqui ainda
//     let s = "olá"; // s agora é válida

//     // faz alguma coisa com s
// } //s passa a ser inválida


fn main() {
    // O tipo String é alocado na heap é capaz de armazenar texto que é desconhecido em tempo de compilação
    // É possível criar uma String de uma string literal usando a função from

    let mut s = String::from("Olá"); // s é valida desse ponto em diante

    // este tipo de string pode ser alterada, ao contrário das strings literais
    // O tipo String é uma cadeia expansiva de caracteres armazenada na heap
    // strings literais são rapidas e eficientes, pois é possível saber o conteúdo em tempo de compilação
    // então o texto é colocado diretamente para dentro do executável final, essas propriedades provêm
    // do aspecto imutável da string literal
    s.push_str(", mundo");

    // faz alguma coisa com s

    println!("{s}"); // s não é mais válida
} //a função especial  drop() é chamada automaticamente ao fechar chaves {}, para retornar a memória 
//  tinha sido alocada
// Essa forma de desalocar memória no fim do tempo de vida útil de um  item as vezes é chamado de Rsource Is
// initialization (RAII, do inglês Aquisição de Recurso É Inicialização.