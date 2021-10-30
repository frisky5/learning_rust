use std::env::var;
use std::io; // this will include the lib standard input output

fn main() {
    let variable_1 = 15; //this variable is immutable (value cannot be changed), by default the numbers are signed 32 bit integers
    println!("variable_1 = {}", variable_1);
    //variable_1 = 30;    this wont be accepted by the compiler as the variable is immutable.
    /*==================================================================================*/
    let mut variable_2 = 25; //this variable is mutable (value can be changed)
    println!("variable_2 = {}", variable_2);
    variable_2 = 30; //this is allowed because the variable is defined as mutable with the mut thingy
    println!("new value of variable_2 = {}", variable_2);

    /*==================================================================================*/
    let variable_3 = 5;
    println!("variable_3 before shadowing = {}", variable_3);
    let variable_3 = variable_3 + 10; // this is called shadowing, and the value will be lost when we exit this code block
    println!("variable_3 after shadowing = {}", variable_3);

    {
        // this a code block inside the code block
        let variable_3 = variable_3 * 10;
        println!(
            "variable_3 inside code block for shadowing = {}",
            variable_3
        )
    }
    println!("variable_3 after code block for shadowing = {}", variable_3);

    /*==================================================================================*/
    let variable_4: u8 = 129; // this is a scalar type, unsigned int of 8 bit
                              //let variable_4: i8 = 129;  this is not allowed because signed 8 bits cannot fit the number 129
    let variable_4: i32 = 654654654; // this is shadowing, we can change data type of the same variable name with shadowing
    let variable_4: usize = 123123123; // this variable type is unsigned arch size (32 bit or 64 bit cpu)
    let variable_4: isize = 123123123; // this variable type is singed arch size (32 bit or 64 bit cpu)
    let variable_4: f64 = 65134.16465846; // this is floating point
    let variable_4: bool = false; // this is boolean type of true and false
    let variable_4 = 'F'; // this is character type, four bytes in size and represents a Unicode Scalar
    let variable_4 = '😻';
    println!("variable_4 after all the shadowing = {}", variable_4);

    /*==================================================================================*/
    println!("Enter some text :");
    let mut user_input = String::new(); // create mutable variable of type string
    io::stdin().read_line(&mut user_input); // this will wait for the user to enter text and press enter (new line), user_input is passed by reference.
                                            // references are immutable by default so in order to pass by reference and make the pass mutable you must do & mut var

    user_input = String::new(); // because read line will append to the string you gave it reference to

    println!("Enter a number :");
    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read line"); // read line returns a io::Result, which is an enum of type usize

    let number_from_string: isize = user_input
        .trim()
        .parse()
        .expect("Failed to parse text you entered as a number");

    /*==================================================================================*/
    function_1(number_from_string);
    function_2(5, String::from("Mahmood"));

    let variable_6 = function_3(200);
    println!("variable_6 = {}", variable_6);
    /*==================================================================================*/
    let mut variable_7 = 30;
    variable_7 = 50;
    if variable_7 == 50 {
        println!("if variable_7 = {}", variable_7);
    } else {
        println!("else variable_7 = {}", variable_7);
    }

    let variable_8 = if variable_7 == 50 { 30 } else { 50 };
    println!("variable_8 = {}", variable_8);
}

fn function_1(x: isize) {
    println!("this is function_1, you passed : {}", x);
    println!("this is function_1, you passed*10 : {}", x * 10);
}
fn function_2(x: isize, y: String) {
    println!("this is function_2, you passed : {},{}", x, y);
}

fn function_3(x: isize) -> isize {
    return x * 500;
    //or just type x*500 without semicolon
}
