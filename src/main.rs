use fibonacci::*;
use std::error::Error;
use std::io::{self, Write};

fn main() -> Result<(), Box<dyn Error>> {
    println!("Sequência de Fibonacci:\n");

    for n in 1..=10 {
        let fibo = fibonacci(n - 1).unwrap_result();
        println!("fibonacci({n:>2}): {fibo}");
    }

    println!();

    print!("Para obter o valor de fibonacci(n) digite n: ");

    io::stdout().flush()?;

    // Cria uma nova String para armazenar a entrada do usuário
    let mut input = String::new();

    // Lê a entrada do usuário e trata possíveis erros
    std::io::stdin().read_line(&mut input)?;

    // Remove espaços em branco e converte a string para um número inteiro
    let result = input.trim().parse::<u32>().map_err(|parse_err| {
        // Add a custom error message
        FibonacciError::InvalidInput(input, parse_err)
    });

    match result {
        Ok(n) if n > 0 => {
            let fibo = fibonacci(n - 1).unwrap_result();
            println!("\nfibonacci({n:>2}): {fibo}\n");
        }
        Ok(_) => eprintln!("\nDigitar n > 0\n"),
        Err(e) => eprintln!("{e}"),
    }

    Ok(())
}
