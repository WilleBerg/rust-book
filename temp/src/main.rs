fn main() {
    let mut test: i32 = 0;
    test = convert_fahrenheit(100);
    println!("{} degrees F is {} degrees C", 100, test);
    
    test = convert_celcius(30);

    println!("{} degrees C is {} degrees F", 30, test);
}

fn convert_fahrenheit(fahrenheit: i32) -> i32 {
    let celcius = (fahrenheit - 32) * 5/9;
    return celcius;
}

fn convert_celcius(celcius: i32) -> i32 {
    let fahrenheit = (celcius * 9/5) + 32;
    return fahrenheit;
}

