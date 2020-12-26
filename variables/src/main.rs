
fn main() {
    mut_and_immutable();
    var_and_constants();
    shadowing();
}

fn mut_and_immutable() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is {}", x);
}

fn var_and_constants() {
    const MAX_POINTS: u32 = 100_000;

    println!("The value of MAX_POINTS is {}", MAX_POINTS)
}

fn shadowing() {
    let x = 5;

    let x = x + 1;

    let x = x * 2;

    println!("The value of x is: {}", x);
}