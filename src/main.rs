fn main() {
    let variable_1 = 15; //this variable is immutable (value cannot be changed)
    println!("variable_1 = {}", variable_1);
    //variable_1 = 30;    this wont be accepted by the compiler as the variable is immutable.

    let mut variable_2 = 25; //this variable is mutable (value can be changed)
    println!("variable_2 = {}", variable_2);
    variable_2 = 30; //this is allowed because the variable is defined as mutable with the mut thingy
    println!("new value of variable_2 = {}", variable_2);
}