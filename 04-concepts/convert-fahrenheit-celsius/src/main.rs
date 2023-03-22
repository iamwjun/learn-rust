fn main() {
    println!("Hello, world!");
    let celsius = fahrenheit_to_celsius(100.0);
    let fahrenheit = celsius_to_fahrenheit(35.0);
    println!("100 F is {celsius} C, 35 C is {fahrenheit} F");
}

fn fahrenheit_to_celsius(fahrenheit: f32) -> f32 {
    return  (fahrenheit - 32.0) * 5.0 / 9.0;
}

fn celsius_to_fahrenheit(celsius: f32) -> f32 {
    return celsius * 9.0 / 5.0 + 32.0;
}