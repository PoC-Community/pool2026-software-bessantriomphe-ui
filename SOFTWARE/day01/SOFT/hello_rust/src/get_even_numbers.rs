pub fn get_even_numbers(numbers: &[i32]) -> String {
        numbers.iter().copied()             
        .filter(|&x| x>0)
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(" - ")
}
