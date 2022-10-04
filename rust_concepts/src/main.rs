// use std::io;

// const GLOBAL_CONSTANT:u32 = 5;

fn main() {
    let mut s = String::from("hello world!");
    
    let word = second_word(&s);

    println!("{word}");
    s.clear();
}

/**
 * write a function that takes a string of words separated 
 * by spaces and returns the first word it finds in that string. 
 * If the function doesn’t find a space in the string, the whole 
 * string must be one word, so the entire string should be returned
 */
// fn first_word(s: &String) -> usize {
//     let bytes = s.as_bytes();
//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return i;
//         }
//     }
//     s.len()
// }

fn second_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

// fn dangling_reference() -> &String {
//     let s = String::from("hello");
//     &s
// }

// fn loop_with_label() {
//     let mut count = 0;
//     'counting_up: loop {
//         println!("Count = {count}");
//         let mut remaining = 10;

//         loop {
//             println!("remaining = {remaining}");
//             if remaining == 9 {
//                 break;
//             }

//             if count == 2 {
//                 break 'counting_up;
//             }

//             remaining -= 1;
//         }
//         count += 1;
//     }
// }

// fn loop_function() {
//     let mut counter = 0;
//     let result = loop {
//         counter += 1;
//         if counter == 10 {
//             break counter *2;
//         }
//     };
//     println!("{result}");
// }

// fn if_else() {
//     let number = 3;
//     if number < 5 {
//         println!("{}", number != 5);
//         println!("{}", number == 5);
//     } else {
//         println!("condition was false");
//     }
// }


// fn with_return(x: u32) -> u32 {
//     x+1
// }

// fn std_input_output() {
//     let mut number = String::new();

//     io::stdin()
//         .read_line(&mut number)
//         .expect("Failed to read line");
//     let number: usize = number
//         .trim()
//         .parse()
//         .expect("Not a number");
//     println!("your number is {number}");
// }

// fn data_types() {
    
//     let int: u128 = 99999999999999;
//     let float: f64 = 0.99999999999999;
//     let char: char = '😊';
    
//     let tup: (i32, f64, u8) = (500, 6.4, 1);
//     let tup_data1 = tup.0;

//     let array = [0, 1, 2, 3, 4];
//     let array_data = array[0];
//     println!("integer {int}");
//     println!("floating {float}");
//     println!("character {char}");
    
//     println!("tuple {tup_data1}");
//     println!("array {array_data}");
// }

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