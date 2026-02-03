mod get_even_numbers;
mod what_is_your_name;
mod models;
mod repositories;
mod views;
mod controller;
mod router;

fn main() {
    let array = vec![1, 0, 19, 17, 16, 8, 13, 24];
    println!("{}", get_even_numbers::get_even_numbers(&array));
    let name = what_is_your_name::what_is_your_name();
    println!("Hello {}", name);

    router::router()
}
