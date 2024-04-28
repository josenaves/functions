fn main() {
    let name = String::from("José");

    println!("::: Functions");

    println!("::: get_greetings: {}", get_greeting());
    println!("::: get_greetings: {}", get_greeting_2(&name));
    println!("::: add_numbers(2, 5): {}", add_numbers(2, 5));

    println!("::: Optional paramenter - empty: {:?}", greet(None));
    println!("::: Optional parameter - Naves: {:?}", greet(Some(&name)));
}

fn get_greeting() -> String {
    return String::from("Hello, Rust!")
}

fn get_greeting_2(name: &str) -> String {
    let greeting = format!("Hello, {}!", name);
    return greeting;
}

fn add_numbers(x: i32, y: i32) -> i32 {
    let sum = x + y;
    sum
}

fn greet(name: Option<&str>) -> String {
    match name {
        Some(n) => return format!("Hello, {}!", n),
        None => return String::from("Hello, stranger!"),
    }
}