fn main() {
    println!("Hello, world!");

    another_function();

    function_with_params(5);

    let x = five();
    
    println!("The value returned from the function is {}", x);

    let y = add_one(10);

    println!("The value after add one is: {}", y);
}

//Simple function
fn another_function() {
    println!("From another function");
}

//Simple function with params
fn function_with_params(x: i32) {
    println!("The value of x is: {}", x);
}

//Function that returns a value
//We specify the type of the return value 
//after -> 
//rust return values are the final expression in the body
//of the function not statements (notice no semicolo)
fn five() -> i32 {
    5
}

//return with params
fn add_one(x: i32) -> i32 {
    x + 1
}
