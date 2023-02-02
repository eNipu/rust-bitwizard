use std::io;

#[allow(dead_code)]
pub fn variables() {
    let x = 5;
    println!("The value of x is: {x}");
    // x = 6;
    // println!("The value of x is: {x}");
}

#[allow(dead_code)]
pub fn shadowing() {
    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x in the inner scope is = {x}");
    }

    println!("The value of x is: {x}");

    // let spaces = "   ";
    // let spaces = spaces.len(); //allowed

    // let mut spaces = "   ";
    // spaces = spaces.len(); // not allowed
}
#[allow(dead_code)]
pub fn math_operation() {
    // addition
    let sum = 5 + 10;
    println!("The value of sum is: {sum}");

    // subtraction
    let difference = 95.5 - 4.3;
    println!("The value of difference is: {difference}");

    // multiplication
    let product = 4 * 30;
    println!("The value of product is: {product}");

    // division
    let quotient = 56.7 / 32.2;
    println!("The value of quotient is: {quotient}");
    let truncated = -5 / 3; // Results in -1
    println!("The value of truncated is: {truncated}");

    // remainder
    let remainder = 43 % 5;
    println!("The value of remainder is: {remainder}");
}

#[allow(dead_code)]
pub fn compount_types() {
    //tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    // destructuring tuple: break tuple into individual parts
    let (x, y, z) = tup;
    println!("The value of y is: {y}");
    println!("The value of z is: {z}");
    println!("The value of x is: {x}");

    // access tuple elements directly by using a period (.) followed by the index of the value we want to access
    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;
    println!("The value of five_hundred is: {five_hundred}");
    println!("The value of six_point_four is: {six_point_four}");
    println!("The value of one is: {one}");

    //unit tuple
    let tup: () = ();
    println!("The value of unit tup is {:?}", tup);

    //array: every element must have the same type
    let a = [1, 2, 3, 4, 5];
    let first = a[0];
    let second = a[1];
    println!("The value of first is: {first}");
    println!("The value of second is: {second}");

    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    let jan = months[0];
    println!("The value of jan is: {jan}");

    let a = [2; 4];
    println!("The value of a is: {:?}",a);
}


pub fn access_array() {
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}

#[allow(dead_code)]
pub fn functions() {
    let x = 5;
    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of y is: {y}");
}

pub fn statements_and_expressions() {
    // Statements are instructions that perform some action and do not return a value.
    // Expressions evaluate to a resulting value.
    // let y = (let x = 6); // not allowed
    let x = 5;
    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of y is: {y}");
}

pub fn functions_with_return_values() -> i32 {
    let x = 5;
    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of y is: {y}");
    y
}

#[allow(dead_code)]
pub fn return_value_from_loop() -> i32 {
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The value of result is: {result}");
    result
}