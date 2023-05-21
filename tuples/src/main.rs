fn main() {
    let tup: (i32, f64, bool) = (500, 6.4, true);

    // destructuring
    let (x, y, z) = tup;

    println!("The value of y is: {}", y);
    println!("The value of x is: {}", x);
    println!("The value of z is: {}", z);

    let x = tup.1;
    let y = tup.2;
    let z = tup.0;

    println!("The value of y is: {}", y);
    println!("The value of x is: {}", x);
    println!("The value of z is: {}", z);
}
