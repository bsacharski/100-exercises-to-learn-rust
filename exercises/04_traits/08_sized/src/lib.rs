pub fn example() -> usize {
    // Trying to get the size of a str (or any other DST)
    // via `std::mem::size_of` will result in a compile-time error.
    //
    // TODO: Comment out the following line and move on to the next exercise.
    // ME: but &str does have know size, so let's try it
    std::mem::size_of::<&str>()
}

#[cfg(test)]
mod tests {
    use crate::example;

    #[test]
    fn size_of_str_ptr() {
        assert_eq!(example(), 16);
    }
}
