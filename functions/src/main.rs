fn main() {
    println!("Hello, world!");

    // Statement vs Expression
    let x = {
        let y = 4;
        y + 5 
    };
    another_function(x);
    print_labeled_measurement(5, 'm');
    println!("{}", five());
}

fn another_function(x : i32) {
    println!("Parameter x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn five() -> i32 {
    5
}
