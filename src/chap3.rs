
#[allow(dead_code)]
pub fn variables(){
    let x = 5;
    println!("The value of x is: {x}");
    // x = 6;
    // println!("The value of x is: {x}");
}


pub fn shadowing(){
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
