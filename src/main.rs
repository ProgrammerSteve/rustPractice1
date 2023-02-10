//input output library is brought into the scope
//the io library comes from the standard libary known as std
//By default, Rust has a set of items defined in the standard library
//that it brings into the scope of every program
//This set is called the prelude, and you can see everything in it in the standard library documentation.
//If a type you want to use isn’t in the prelude, you have to bring that type into scope explicitly with a use statement.
use std::io;

//fn declares a new function
fn main() {
    //println macro is called. "!" indicates a macro is being called
    println!("Guess the number");
    println!("Please input your guess.");

    //let creates a variable
    //in Rust, variables are immutable by default
    //to make variable mutable, "mut" is added after let
    //String::new is a function that returns a new instance of a string
    //the :: syntax in ::new indicates that new is associated function of the String type
    //an associated function is a function that's implemented on a type, in this case String
    let mut guess = String::new();

    //we included the io module from the std standard library
    //now we call the stdin function from the io module
    //alternatively, std::io::stdin() could be used
    //.read_line() calls the read_line method, &mut guess is the argument
    // to tell what string to store the user input in

    //& indicates that this argument is a reference, which gives you a way to
    //let multiple parts of the code access one piece of data without needing
    //to copy that data into memory multiple times

    //like variables, references are immutable by default,
    //hence you need to write &mut to make it mutable

    //the Result given back by the read_line method is an enumeration, or enum.
    //a type which can be one of the multiple possible states
    //we call each possible state a variant
    //the purpose of the Result types is to encode error-handling information
    //Result's variants are Ok and Err
    //Values of the Result type have methods defined on them
    //an instance of Result has an expect method you can call
    //if Result is an Err value, expect will cause the program to crash and display the message
    //you passed as an argument to the expect method

    //if you don't add the expect method to a result type,
    //you will get a warning, but the code will compile
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    //prints the value for guess
    //the {} set of curly brackets is a placeholder
    //when printing a variable it can go in the curly brackets,
    //when printing the result of an expression, add a comma and list the expression
    //example:
    //let x = 5;
    //let y = 10;
    //println!("x = {x} and y + 2 = {}", y + 2);
    //returns x = 5 and y = 12.

    println!("You guessed: {guess}");
}
