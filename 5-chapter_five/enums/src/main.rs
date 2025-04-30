// Enums permitem definir um tipo por meio da enumeração de seus possíveis valores
// Nós podemos enumerar todos os possíveis valores, é daí que vem o nome enumeração.
//Ex: um endereço ip pode ser da versão 6, ou 4, nunca das duas ao mesmo tempo

// Esta propriedade dos endereços IP faz com que a enum seja bem apropriada para 
// este caso, pois enums só podem assumir o valor de uma de suas variantes.

enum VersaoIp { //VersaoIp agora é um tipo de dado que podemos usar em qualquer                
    V4,         //lugar do nosso código
    V6,
}

//v4 e v6 compartilha as mesmas coisas, porém são diferentes

struct EnderecoIp {
    versao: VersaoIp,
    endereco: String,
    
}


fn main() {
    let ipv4 = VersaoIp::V4; //v4 e v6 são do mesmo tipo, são IPs
    let ipv6 = VersaoIp::V6;

    
    rotear(ipv4); //agora podemos chamar a função com qualquer um das
    // duas variantes da enum
    rotear(ipv6);
    
    let loopback_ipv4 = EnderecoIp {
        versao:VersaoIp::V4,
        endereco:String::from("127.0.0.1"),
    };

    let loopback_ipv6 = EnderecoIp {
        versao: VersaoIp::V6,
        endereco: String::from("::1"),
    };
}

//agora podemos fazer uma função que usa VersãoIp

fn rotear(versao_ip: VersaoIp) {

}