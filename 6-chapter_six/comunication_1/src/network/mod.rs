//quando mod.rs está dentro de uma pasta, ele quer dizer nome_da_pasta.rs
//veja lib.rs
// lá network tinha o próprio arquivo, aí depois que declaramos server dentro de
//network (que no caso é esse arquivo que foi renomeado e movido para a pasta network)
//ele precisava estar dentro de uma pasta chamada network

// esse arquivo, mod.rs era o network.rs que estava em src, aí depois que eu movi ele pra cá
//ele precisou ser chamado de mod.rs para que lib.rs reconhecesse essa pasta como um
//módulo

pub fn connect() {}
pub mod server;
