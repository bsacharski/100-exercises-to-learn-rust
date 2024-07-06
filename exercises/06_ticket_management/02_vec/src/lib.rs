// Given a number `n`, return the `n+1`th number in the Fibonacci sequence.
//
// The Fibonacci sequence is defined as follows:
//
// - The first number of the sequence is 0.
// - The second number of the sequence is 1.
// - Every subsequent number is the sum of the two preceding numbers.
//
// So the sequence goes: 0, 1, 1, 2, 3, 5, 8, 13, 21, and so on.
//
// We expect `fibonacci(0)` to return `0`, `fibonacci(1)` to return `1`,
// `fibonacci(2)` to return `1`, and so on.

static mut FIBONACCI_NUMBERS: Vec<u32> = Vec::new();

pub fn fibonacci(n: u32) -> u32 {
    let idx = usize::try_from(n).unwrap();

    unsafe {
        if FIBONACCI_NUMBERS.len() == 0 {
            FIBONACCI_NUMBERS.push(0);
            FIBONACCI_NUMBERS.push(1);
            FIBONACCI_NUMBERS.push(1);
        }

        let result = FIBONACCI_NUMBERS.get(idx);
        if result.is_none() {
            let a = fibonacci(n - 2);
            let b = fibonacci(n - 1);
            FIBONACCI_NUMBERS.push(a + b);
        }

        return FIBONACCI_NUMBERS[idx];
    }
}

#[cfg(test)]
mod tests {
    use crate::fibonacci;

    #[test]
    fn first() {
        assert_eq!(fibonacci(0), 0);
    }

    #[test]
    fn second() {
        assert_eq!(fibonacci(1), 1);
    }

    #[test]
    fn third() {
        assert_eq!(fibonacci(2), 1);
    }

    #[test]
    fn tenth() {
        assert_eq!(fibonacci(10), 55);
    }

    #[test]
    fn thirthieth() {
        assert_eq!(fibonacci(30), 832040);
    }
}
