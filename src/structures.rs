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

pub fn fibonacci(integer: u32) -> Result<u64, FibonacciError> {
    if integer == 0 {
        return Ok(0);
    } else if integer == 1 {
        return Ok(1);
    }

    let mut fib_iter = Fibonacci::new();
    let mut result = 0;

    for _ in 0..integer {
        match fib_iter.next() {
            Some(num) => result = num,
            None => return Err(FibonacciError::Overflow(integer)),
        }
    }

    Ok(result)
}
