//Definição de modulo: mod nome_do_modulo {}
mod network {
    //namespace network
    //as duas funções connect estão em namespaces diferentes
    fn connect() {}

  //podemos ter módulos dentro de módulos para uma melhor organização
    mod client { //namespace client
        fn connect() {}
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
