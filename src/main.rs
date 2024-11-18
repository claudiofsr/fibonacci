use fibonacci::*;
use std::error::Error;
use std::io::{self, Write};

fn main() -> Result<(), Box<dyn Error>> {
    println!("Sequência de Fibonacci:\n");

    for n in 0..=10 {
        let fibo = fibonacci(n).unwrap_result();
        println!("O {n}º número na sequência de Fibonacci é {fibo}");
    }

    println!();

    println!("Obter a sequência de Fibonacci número N: ");
    print!("Digite o número inteiro N: ");

    io::stdout().flush()?;

    // Cria uma nova String para armazenar a entrada do usuário
    let mut input = String::new();

    // Lê a entrada do usuário e trata possíveis erros
    std::io::stdin().read_line(&mut input)?;

    // Remove espaços em branco e converte a string para um número inteiro
    let number = input
        .trim()
        .parse::<u32>()
        .map_err(|parse_err| {
            // Add a custom error message
            FibonacciError::InvalidInput(input, parse_err)
        })
        .unwrap_result();

    let fibo = fibonacci(number).unwrap_result();
    println!("\nO {number}º número na sequência de Fibonacci é {fibo}\n");

    Ok(())
}
