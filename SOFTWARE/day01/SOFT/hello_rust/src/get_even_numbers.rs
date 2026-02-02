pub fn get_even_numbers(numbers: &[i32]) -> String {
        numbers.iter()                     
        .filter(|&x| x%2==0)
        .map(|&x| x.to_string())
        .collect::<Vec<String>>()
        .join(" - ")
}
