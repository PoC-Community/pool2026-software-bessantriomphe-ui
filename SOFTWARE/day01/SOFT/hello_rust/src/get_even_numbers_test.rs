#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_even_numbers_positive() {
        let numbers = vec![1, 2, 3, 4, 5, 6];
        assert_eq!(get_even_numbers(&numbers), "2 - 4 - 6");
    }
}