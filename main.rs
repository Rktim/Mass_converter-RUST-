use std::io;

fn main() {
    println!("Enter weight in Kilograms: ");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let kilograms: f64 = input.trim().parse().expect("Please enter a valid number");

    let pounds = kilograms / 0.45359237;
    let grams = kilograms * 1000.0;

    println!("Weight in pounds: {} pounds", pounds);
    println!("Weight in grams: {} grams", grams);
}