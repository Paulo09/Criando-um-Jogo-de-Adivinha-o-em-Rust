
Criando um Jogo de Adivinhação em Rust
Entendendo o Problema:

Queremos criar um programa que:

Gere um número aleatório.
Peça ao usuário para adivinhar esse número.
Compare o palpite do usuário com o número aleatório e informe se o palpite está alto, baixo ou correto.
Repita o processo até que o usuário acerte o número.
Solução em Rust:

Rust
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

        let guess: u32 = match guess.trim().parse()   
 {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Você digitou: {}", guess);

        match guess.cmp(&secret_number)   
 {
            std::cmp::Ordering::Less => println!("É um número maior!"),
            std::cmp::Ordering::Greater => println!("É um número menor!"),
            std::cmp::Ordering::Equal => {
                println!("Você acertou! O número secreto era {}", secret_number);
                break;
            }
        }
    }
}
Use o código com cuidado.

Explicação:

Importando módulos: Importamos os módulos rand para gerar números aleatórios e std::io para realizar operações de entrada e saída.
Gerando o número secreto: Geramos um número aleatório entre 1 e 100 e o armazenamos na variável secret_number.
Loop principal:
Lendo o palpite: Pedimos ao usuário para digitar um palpite e o armazenamos na variável guess.
Convertendo para número: Convertemos a entrada do usuário para um número inteiro. Se a conversão falhar, o loop continua sem realizar nenhuma comparação.
Comparando os números: Usamos o método cmp para comparar o palpite com o número secreto e imprimimos uma mensagem indicando se o palpite está alto, baixo ou correto.
Encerrando o loop: Se o palpite estiver correto, imprimimos uma mensagem de vitória e encerramos o loop.
Pontos importantes:

Tratamento de erros: O código verifica se a entrada do usuário é um número válido. Se não for, o loop continua.
Clareza: O código é bem estruturado e comentado, facilitando a compreensão.
Reutilização: O código pode ser facilmente adaptado para gerar números aleatórios em diferentes intervalos ou para implementar diferentes tipos de jogos.
Como executar:

Salvar o código: Salve o código acima em um arquivo com extensão .rs, por exemplo, adivinhacao.rs.
Compilar e executar: Abra um terminal, navegue até o diretório onde você salvou o arquivo e execute o seguinte comando:
Bash
rustc adivinhacao.rs
Use o código com cuidado.

Em seguida, execute o programa:
Bash
./adivinhacao
Use o código com cuidado.

Melhorias:

Limitar o número de tentativas: Você pode adicionar uma variável para contar o número de tentativas e encerrar o jogo após um determinado número de tentativas.
Dificuldade ajustável: Permita que o usuário escolha o intervalo de números para adivinhar.
Múltiplas rodadas: Implemente uma opção para que o jogador possa jogar novamente.
Com este código básico, você já tem um jogo de adivinhação funcional em Rust. Explore as possibilidades e divirta-se personalizando o jogo!
