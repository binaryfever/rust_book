fn main() {
    //variables are immutable by default
    //the below code won't compile
    //let x = 5;
    //println!("The value of x is: {}", x);
    //x = 6;
    //println!("The value of x is: {}", x);

    //use the mut keyword to make a variable 
    //mutable
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    //but immutable variables aren't constants
    //constants always be constant and can't
    //be set from a function call or any other
    //value set during runtime
    //const MAX_POINTS: u32 = 100_000;

    //Variable shadowing is when we create a new
    //variable with the same name as the previous
    //variable assigning it a new value.
    //The new variable is used over the old
    let y = 5;
    let y = y + 1;
    let y = y * 2;
    println!("The value of y is: {}", y);

    //Shadowing lets us change the type because
    //we are basically creating a new variable
    let spaces = "    ";
    let spaces = spaces.len();
    println!("This number of spaces is {}", spaces);

    //But if we did this with a mutable 
    //variable we woud get a compile time
    //error
    //let mut spaces = "    "
    //spaces = spaces.len()
    
}

