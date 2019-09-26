// parser de string qui lance des dés à notre place
// il peut également ajouter des bonus ou retirer des malus

#[macro_use]
extern crate clap;
extern crate rand;
use clap::App;
pub mod dice;
use dice::Dice;

fn main() {
    let yaml = load_yaml!("cli.yml");

    let matches = App::from_yaml(yaml).get_matches();

    let faces = matches
        .value_of("faces")
        .unwrap_or_default()
        .parse::<u32>()
        .unwrap();
    let number = matches
        .value_of("NUMBER")
        .unwrap_or_default()
        .parse::<u32>()
        .unwrap();

    let dice = Dice::new(faces);

    let mut sum = 0;
    println!("You are rolling a dice with {} faces", faces);
    println!("---------");

    let mut r: u32;
    for _i in 0..number {
        r = dice.roll();
        println!("You rolled a {}", r);
        sum += r;
    }

    println!("---------");
    println!("The sum of your {} rolls is {}", number, sum);

    let avg: f64 = sum as f64 / number as f64;
    println!("Your average roll is {}", avg);
}
