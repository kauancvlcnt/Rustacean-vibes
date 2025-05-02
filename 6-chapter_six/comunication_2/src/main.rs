use communicator::client;

/*
No geral, estas são as regras para a visibilidade do item:

    Se um item for público, ele pode ser acessado através de qualquer um 
    dos seus módulos pais.
    Se um item é privado, ele só pode ser acessado por seu módulo pai
    imediato e    qualquer um dos módulos filhos do pai.

*/




//importa nossa biblioteca, o nome da biblioteca é o que fica no cargo.toml

fn main() {
    client::connect();
}