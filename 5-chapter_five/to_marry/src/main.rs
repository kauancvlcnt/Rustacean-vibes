#[derive(Debug, PartialEq)]
enum CivilState {
    Married,
    Single,
}
#[derive(Debug, PartialEq)]
enum Gender {
    Male,
    Female,
}

#[derive(Debug)]
struct Pessoa {
    name: String,
    age: u32,
    civil_state: CivilState,
    gender: Gender,
}

#[derive(Debug)]

enum Couple{
    Couple { woman: Pessoa, man: Pessoa },
    NotPermited,
}

impl Pessoa {
    fn to_maried(mut wife: Pessoa, mut husband: Pessoa) -> Couple {
        //conseguimos dizer aí em cima que wife e husband é mutável pq agora to_maried tem a posse dos dois
        // mary e  jack foram transferidos para cá
        let diffent_gender = wife.gender != husband.gender;
        let is_single =
            wife.civil_state == CivilState::Single && husband.civil_state == CivilState::Single;

        if diffent_gender && is_single {

            wife.civil_state = CivilState::Married;
            husband.civil_state = CivilState::Married;
            Couple::Couple {
                woman: wife,
                man: husband,
            }
        } else {
            Couple::NotPermited
        }
    }
}
fn main() {
    let husband: Pessoa = Pessoa {
        name: String::from("Jack"),
        age: 23,
        civil_state: CivilState::Single,
        gender: Gender::Female,
    };

    let wife = Pessoa {
        name: String::from("Mary"),
        age: 18,
        civil_state: CivilState::Single,
        gender: Gender::Male,
    };

   let couple_mary_and_jack = Pessoa::to_maried(wife, husband);

   println!("{:#?}", couple_mary_and_jack);
    // println!("{:#?} {:#?}", mary.gender, jack.gender); a fn to_maried agora tem a posse de mary e jack
    
}
