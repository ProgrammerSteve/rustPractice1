//IMPORTING COMMENT
//input output library is brought into the scope
//the io library comes from the standard libary known as std
//By default, Rust has a set of items defined in the standard library
//that it brings into the scope of every program
//This set is called the prelude, and you can see everything in it in the standard library documentation.
//If a type you want to use isn’t in the prelude, you have to bring that type into scope explicitly with a use statement.
//uses the rand library that was imported, gets the Rng "trait" which will be discussed further in ch10
//the Rng trait defines methods that random number generators implements
//std::cmp::Ordering is brought into the scope.
//The Ordering type is another enum and has the variants Less, Greater, and Equal

use rand::Rng;
use std::cmp::Ordering;
use std::io;

//FUNCTION COMMENT
//fn declares a new function
//println macro is called. "!" indicates a macro is being called
fn main() {
    println!("Guess the number");

    //RNG COMMENT
    //First the rand::thread_rng() function is called that gives us the particular random number generator we're going to use
    //one that is local to the current thread of execution and is seeded by the operating system
    //Then the gen_range method is called on the random number generator, which takes a range expression as an argument
    //The kind of range expression we're using here takes the form start..=end and is inclusive on the lower and upper bounds
    //therefore 1..=100 is to request a number 1-100
    //Then we print the generator number in the next line
    let secret_number = rand::thread_rng().gen_range(1..=100);
    // println!("The secret number is: {secret_number}");

    //LOOP COMMENT
    //the look keyword is used to create infinite loops.
    //a break statement is needed to break out of the loop

    loop {
        println!("Please input your guess.");
        //VARIABLE COMMENT:
        //let creates a variable
        //in Rust, variables are immutable by default
        //to make variable mutable, "mut" is added after let
        //String::new is a function that returns a new instance of a string
        //the :: syntax in ::new indicates that new is associated function of the String type
        //an associated function is a function that's implemented on a type, in this case String
        let mut guess = String::new();
        //REFERENCE/RESULT COMMENT
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

        //STRING TO NUMBER COMMENT
        //Rust allows us to shadow the previous value of guess with a new one
        //Shadowing lets us reuse the guess variable name rather than using a unique one
        //This is a feature that is often used when you want to convert a value from one type to another
        //guess refers to the original guess value which is a string
        //the trim method gets rid of any whitespace at the beginning or end
        //when the user presses enter for the read_line method, a newline character is added to the string
        //The parse method on strings converse a string to another type, here we use it for string to number
        //we need to tell Rust the exact number type we want with let guess: u32, this is for
        //unsigned 32-bit integer type, which is good for small positive numbers
        //secret number will be inferred by Rust to be of the type u32 also when comparing
        //if the string cannot be parsed into a number, an Err variant for the Result is thrown so the expect method
        //is needed to handle the Result variant
        //no input handling: let guess: u32 = guess.trim().parse().expect("Please type a number!");
        //switching from an expect call to a match expression to handle the error
        //Parse returns a Return type with the variants Ok and Err. We are using a match expression here
        //If parse is successful, it will return an Ok value that contains the resultant number. That Ok value
        // will match the first arm's pattern and the match expression will just return a num value
        //if parse is unsuccessful, it will return an Err value that contains more information about the error.
        //the underscore _ is a catchall value. In this expression we're saying we want to mall all Err values,
        //no matter what information they have inside them. When the Err arm is matched, the continue is called,
        //effectively the program ignores all errors that parse might encounter

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        //PRINTING COMMENT
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

        //COMPARING COMMENT
        //the cmp method compares two values and can be called on anything that can be compared.
        //It takes a reference to whatever you want to compare with: here it's comparing guess to secret_number
        //Then it returns a variant of the Ordering enum we brought into the scope
        //We use a match expression to decide what to do next based on the variant of Ordering was returned
        //from the call to cmp

        //MATCH COMMENT
        //A match expression is made up of arms. An arm consists of a pattern to match against, and
        //The code that should be run if the value given to match fits that arm's pattern
        //Rust takes the value given to match and looks through each arm's pattern in turn.
        //we receive an error because rust cannot compare a string with a number type
        //this is due to:  let mut guess = String::new(); Thus we convert the string to a number with:
        //let guess: u32 = guess.trim().parse().expect("Please type a number!");
        //when the Equal arm is satisfied the break statement is called, thus ending the loop
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
    //CRATE COMMENT
    //We will need random number generator to add numbers to guess for
    //Although there is nothing in the standard library that can do we,
    //we can use "rand" crate created by the Rust team
    //the project is a binary crate, meaning it produces an executable file
    //The rand Crate is a library crate, which contains code that is intended to be used in other programs
    //and can't be executed on its own
    //crates can be found at Crates.io where developers posts their libraries for people to use
    //running cargo doc --open will build documentation provided by all your dependencies locally
    //and open it in the browser. The documentation will instruct you how to use the crates
}
