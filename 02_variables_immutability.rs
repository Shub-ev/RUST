// in Rust variables are immutable by default

fn main(){
    // *** declare variables ***
    // 1. let
    // *** immutable ***
    let x = 10;     // this creates immutable variables
    print!("Value of x is : {x}\n");   // print using print macro
    // x = 6;       // as x is immutable reassigning it will cause error
    let x = x + 10;     // we can redeclare a variable using same name  // this will shadow the old one
    println!("New x have value of {x}");
    // creating a scope using a block
    {
        let x = x + 10;     // creating another x which shadows the previous x
        println!("In block value of x is {x}");
        // when this block ends x shadowing ends
    }
    println!("Value of x is {x}");

    // *** mutable ***
    let mut y = 6;  // declaring mutable variable
    println!("Value of y before : {y}");
    y = 10;
    println!("Value of y after : {y}");
    // y = " ";      // gives mismatch type error we are using same previous declaration  the type of prev was int 


    // 2. const 
    // const are by default immutable hence ** mut ** can not be used

    const FIRST_CONSTANT: u32 = 10;
    println!("FIRST_CONSTANT is constant {FIRST_CONSTANT}");

    // FIRST_CONSTANT = 20;    // this is now allowed   // we can not change constant
}