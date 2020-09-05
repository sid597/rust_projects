use std::env;
use std::fs;
fn main() {
    // Note did not use std::env because it is already imported 
    let args: Vec<String> = env::args().collect();
    let (query, filename) = parse_config(&args);
    println!("Searching for {}", query);
    println!("In file {}", filename);
    let contents = fs::read_to_string(filename).expect("something went wrong");
    println!("Contents of file are : \n{}", contents);
}


// Implementation 1 for parse_config
fn parse_config(args:&[String]) -> (&str, &str) {
    /* This rework may seem like overkill for our small program,
     but we’re refactoring in small, incremental steps.*/

    let query = &args[1];
    let filename = &args[2];
    (query, filename)

    /*
    We can take another small step to improve the parse_config function further. At the moment, we’re returning a tuple, but then we immediately break that tuple into individual parts again.
    This is a sign that perhaps we don’t have the right abstraction yet.

    Another indicator that shows there’s room for improvement is the config part of parse_config, which implies that the two values we return are related and are both part of one configuration value.

    We’re not currently conveying this meaning in the structure of the data other than by grouping the two values into a tuple; we could put the two values into one struct and give each of the struct fields a meaningful name. Doing so will make it easier for future maintainers of this code to understand how the different values relate to each other and what their purpose is.
    
    */
}
