//Definição de modulo: mod nome_do_modulo {}
pub mod network {
    //namespace network
    //as duas funções connect estão em namespaces diferentes
    fn connect() {}

    //podemos ter módulos dentro de módulos para uma melhor organização
    pub mod client {
        //namespace client
        pub fn connect() {}
    }
}

#[cfg(test)]
mod tests {
    //  podemos usar super para voltar um módulo na hierarquia a partir de nosso módulo
    use super::network::client;
    #[test] //cargo test pra testar
    fn it_works() {
        //testa o codigo que está dentro dela

        // ::network::client::connect(); podemos indicar com o :: ou super no inicio que
        //  queremos começar pelo modulo raiz
        client::connect();
    }
}

mod outermost {
    pub fn middle_function() {}
    fn middle_secret_function() {}
    mod inside {
        pub fn inner_function() {}
        fn secret_function() {}
    }
}

fn try_me() {
    //outermost não precisa ser público para ser acessado, pois ele está no mesmo modulo
    // que try me agora quem está dentro dele sim
    //veja que inside precisa ser colocado pub para que try_me() possa acessar fora de outermost

    outermost::middle_function;

    //não serão acessados, pois o modulo inside e e algumas
    //funções são privadas
    // outermost::middle_secret_function();
    // outermost::inside::inner_function
    // outermost::inside::secret_function();
}
