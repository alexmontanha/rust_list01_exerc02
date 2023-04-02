use std::io;

pub(crate) fn main() {
    let mut numero_lido: String = String::new();

    println!("Digite um número: ");

    io::stdin().read_line(&mut numero_lido).expect("Erro ao ler o número");

    let numero_tratado = numero_lido.trim();

    match numero_tratado.parse::<i32>() {
        Ok(i) => println!("O número é: {}", i),
        Err(..) => println!("Erro na conversão: {}", numero_tratado),
    };
}
