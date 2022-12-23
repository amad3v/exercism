/// Create an empty vector
pub fn create_empty() -> Vec<u8> {
    vec![]
}

/// Create a buffer of `count` zeroes.
///
/// Applications often use buffers when serializing data to send over the network.
pub fn create_buffer(count: usize) -> Vec<u8> {
    vec![0; count]
}

/// Create a vector containing the first five elements of the Fibonacci sequence.
///
/// Fibonacci's sequence is the list of numbers where the next number is a sum of the previous two.
/// Its first five elements are `1, 1, 2, 3, 5`.
pub fn fibonacci() -> Vec<u8> {
    const COUNT: usize = 5;

    let mut fib = create_buffer(COUNT);

    (0..COUNT).for_each(|idx| match idx {
        i @ (0..=1) => fib[i] = 1,
        _ => fib[idx] = fib[idx - 1] + fib[idx - 2],
    });

    fib
}
