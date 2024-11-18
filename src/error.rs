use std::{
    error::Error,
    fmt::{self, Debug},
    num::ParseIntError,
};

#[derive(Debug)]
pub enum FibonacciError {
    Overflow(u32),
    InvalidInput(String, ParseIntError),
}

impl fmt::Display for FibonacciError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            FibonacciError::Overflow(n) => write!(
                f, 
                "\n\
                Overflow!\n\
                Não foi possível calcular a sequência de Fibonacci.\n\
                Input: {n}\n\
                "
            ),
            FibonacciError::InvalidInput(s, e) => write!(
                f,
                "\n\
                Invalid input: {s}\n\
                Por favor, digite um número inteiro válido.\n\
                ParseIntError: {e}\n\
                "
            ),
        }
    }
}

/// If we want to use std::error::Error in main, we need to implement it for FibonacciError
impl Error for FibonacciError {}
