
fn main() {
    mut_and_immutable();
    var_and_constants();
    shadowing();

    let x = five();
    println!("The value of x is: {}", x);

    let x = plus_one(5);
    println!("The value of x is: {}", x);

    for_loop();
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

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn for_loop() {
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is: {}", element)
    }
}