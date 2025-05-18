use rand::Rng;
use std::{thread::sleep, time};
#[derive(Debug)]
enum Status {
    Chovendo(u8),
    Limpo(u8),
    Status(u8),
}

#[derive(Debug)]
struct Tempo {
    umidade: u8,
    status: Status,
}
impl Tempo {
    fn time_observer(time_simu: (u8, Status)) -> Tempo {
        let (umidade, status) = time_simu;
        Tempo { umidade, status }
    }
}

fn time_simulator() -> (u8, Status) {
    //delay
    let minutes = time::Duration::from_millis(1000);
    sleep(minutes);

    //generate numbers random
    let mut number_rand = rand::rng();
    let number_rand = number_rand.random::<u8>();

    let status = if number_rand > 128 {
        Status::Chovendo(number_rand)
    } else {
        Status::Limpo(number_rand)
    };
    (number_rand, status)
}

fn main() {
    loop {
        let time = Tempo::time_observer(time_simulator());
        println!("{:#?}", time);
    }
}

//o programa deve ficar em pooling
//simule usando um loop e números "aleatórios"

//se a umidade tiver acima de 70 o programa deve fechar o toldo parcialmente
//se a umidade tiver acima de 100  e chovendo, o programa deve fechar completamente
//se a umidade tiver 100 e não tiver chovendo o programa deve abrir o toldo completamte
//se não tiver chovendo e a umidade acima de 120 o toldo deve ser fechado em todos os casos
