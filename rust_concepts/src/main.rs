// const GLOBAL_CONSTANT:u32 = 5;

fn main() {
    shadowing()
}

fn shadowing() {
    let x = 5;
    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x is shadowed by {x}")
    }
    println!("The value of x is: {x}")
}

// fn mutability() {
//     let mut x = 5;
//     println!("the value of x is: {x}");
//     x = 6;
//     print!("the value of x is: {x}");
// }