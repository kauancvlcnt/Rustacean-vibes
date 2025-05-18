// se você quiser abortar no panic no modo de release, adicione isso no Cargo.toml:

fn main() {
    panic!("Quebra tudo"); //causa uma mensagem de erro
    //Usando backtrace (rastro) de panic
    // RUST_BACKTRACE=1 cargo run //mostra o rastro de todo o programa
    // Backtraces em Rust funcionam como em outras linguagens: a chave
    //  para ler o backtrace é começar do topo e ler até você ver os arquivos
    // que você escreveu. Esse é o ponto em que o problema se originou
}
