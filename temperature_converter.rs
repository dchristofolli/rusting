use std::io;

fn main(){
    println!("Type 1 to convert from Fahrenheit to Celsius\n
    Type 2 to convert from Celsius to Fahrenheit");
    let mut option = String::new();
    io::stdin()
    .read_line(&mut option)
    .expect("Failed to read input");
    if option.contains("1"){
        fahrenheit_to_celsius();
    }
    else if option.contains("2"){
        celsius_to_fahrenheit();
    }
    else{
        println!("You have to input 1 or 2");
    }
}
fn fahrenheit_to_celsius(){
    println!("Type the Fahrenheit value to be converted to Celsius");
    let mut fahrenheit = String::new();
    io::stdin()
    .read_line(&mut fahrenheit)
    .expect("Failed to read input");
    let parse_result = fahrenheit.trim().parse();
        let fahrenheit: u32 = match parse_result {
            Ok(num) => num,
            Err(_) => 0,
        };
    let celsius = (fahrenheit - 32) * 5/9;
    println!("{}°C",celsius);
}
fn celsius_to_fahrenheit(){
    println!("Type the Celsius value to be converted to Fahrenheit");
    let mut celsius = String::new();
    io::stdin()
    .read_line(&mut celsius)
    .expect("Failed to read input");
    let parse_result = celsius.trim().parse();
        let celsius: u32 = match parse_result {
            Ok(num) => num,
            Err(_) => 0,
        };
    let fahrenheit = (celsius * 9/5) + 32;
    println!("{}°C",fahrenheit);
}