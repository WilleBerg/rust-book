fn main() {
    let mut counter: u8 = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);

    // for loop

    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("The value is {}", element);
    }

    for number in (1..6).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}
