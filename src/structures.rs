use crate::FibonacciError;

pub struct Fibonacci {
    curr: u64,
    next: u64,
}

impl Fibonacci {
    fn new() -> Fibonacci {
        Fibonacci { curr: 0, next: 1 }
    }
}

impl Iterator for Fibonacci {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        // Tenta calcular o próximo número da sequência
        let new_next = self.curr.checked_add(self.next)?;
        self.curr = self.next;
        self.next = new_next;
        Some(self.curr)
    }
}

pub fn fibonacci(index: u32) -> Result<u64, FibonacciError> {
    let mut fibo = Fibonacci::new();
    let mut result = 0;

    for _ in 0..index {
        match fibo.next() {
            Some(num) => result = num,
            None => return Err(FibonacciError::Overflow(index)),
        }
    }

    Ok(result)
}
