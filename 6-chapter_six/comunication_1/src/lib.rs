// para compilar uma biblioteca devemos utilizar cargo build, ao invés de cargo run

//o modulo ainda está sendo declarado aqui, em lib.rs
//porém ele irá procurar o corpo do modulo em nome_do_modulo.rs

/*Vamos resumir as regras dos módulos em relação aos arquivos:

    Se um módulo chamado foo não possui submódulos, você deve colocar as declarações    para foo em um arquivo chamado foo.rs.
    Se um módulo chamado foo possui submódulos, você deve colocar as declarações    para foo em um arquivo chamado foo/mod.rs.
*/
mod client;

mod network;
