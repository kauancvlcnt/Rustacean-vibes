use std::net::{IpAddr, Ipv6Addr};

fn main() {
    //casos que é interessante usar unwrap:
    //casos quando temos certeza de que o resultado será Ok
    // Ex:
    let home: IpAddr = "192.168.0.1".parse().unwrap();
    // Nós estamos criando uma instância IpAddr ao analisar uma string hardcoded
    // já que a string é hardcoded, não tem nenhuma chance do Result ser Err

    // Panic!
    // É aconselhável fazer que seu código entre em panic! quando é possível
    //  que ele entre em um mau estado. Nesse contexto, mau estado é quando alguma
    //  hipótese, garantia, contrato ou invariante foi quebrada, tal como valores
    //  inválidos, valores contraditórios, ou valores faltando que são passados
    //  a seu código - além de um ou mais dos seguintes:

    //   O mau estado não é algo que é esperado que aconteça ocasionalmente.
    //    Seu código após certo ponto precisa confiar que ele não está nesse mau estado.
    //   Não há uma forma boa de codificar essa informação nos tipos que você usa.
    // Se alguém chama seu código e passa valores que não fazem sentido, a melhor
    //  escolha talvez seja entrar em panic! e alertar a pessoa usando sua biblioteca
    //  do bug no código dela para que ela possa consertá-la durante o desenvolvimento
    //
    //     // loop {
    //         // snip

    //         let palpite: i32 = match palpite.trim().parse() {
    //             Ok(num) => num,
    //             Err(_) => continue,
    //         };

    //         if palpite < 1 || palpite > 100 {
    //             println!("O número secreto vai estar entre 1 e 100.");
    //             continue; chama a próxima iteração do loop
    //         }

    //         match palpite.cmp(&numero_secreto) {
    //         // snip
    //     }
    //
    // um método que pega um dos campos da struct e retorna, é chamdo de getter
    // a função associada new() funciona como um setter, "deixando o codigo privado"
}
