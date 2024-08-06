use rand::Rng;
use std::io;

fn main() {
    println!("Bem-vindo ao jogo de adivinhação!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Digite seu palpite:");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Falha ao ler a linha");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Você digitou: {}", guess);

        match guess.cmp(&secret_number) {
            std::cmp::Ordering::Less => println!("É um número maior!"),
            std::cmp::Ordering::Greater => println!("É um número menor!"),
            std::cmp::Ordering::Equal => {
                println!("Você acertou! O número secreto era {}", secret_number);
                break;
            }
        }
    }
}
