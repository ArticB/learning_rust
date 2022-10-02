// const GLOBAL_CONSTANT:u32 = 5;

fn main() {
    data_types();
}

fn data_types() {
    
    let int: u128 = 99999999999999;
    let float: f64 = 0.99999999999999;
    let char: char = '😊';
    
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let tup_data1 = tup.0;

    let array = [0, 1, 2, 3, 4];
    let array_data = array[0];
    println!("integer {int}");
    println!("floating {float}");
    println!("character {char}");
    
    println!("tuple {tup_data1}");
    println!("array {array_data}");
}

// fn shadowing2() {
//     let x = "hello";

//     {
//         let x = x.len(); 
//         println!("The length of x string is {x}");
//     }
//     println!("{x}");
// }

// fn shadowing() {
//     let x = 5;
//     let x = x + 1;

//     {
//         let x = x * 2;
//         println!("The value of x is shadowed by {x}")
//     }
//     println!("The value of x is: {x}")
// }

// fn mutability() {
//     let mut x = 5;
//     println!("the value of x is: {x}");
//     x = 6;
//     print!("the value of x is: {x}");
// }