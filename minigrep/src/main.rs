use std::env;
use std::fs;
fn main() {
    // Note did not use std::env because it is already imported 
    let args: Vec<String> = env::args().collect();
    let config = parse_config(&args);
    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);
    let contents = fs::read_to_string(config.filename).expect("something went wrong");
    println!("Contents of file are : \n{}", contents);
}

/*
/* Version 1 for parse_config */

fn parse_config(args:&[String]) -> (&str, &str) {
    /* This rework may seem like overkill for our small program,
     but we’re refactoring in small, incremental steps.*/

    let query = &args[1];
    let filename = &args[2];
    (query, filename)

    /*
    We can take another small step to improve the `parse_config` function further. At the moment, we’re returning a tuple, but then we immediately break that tuple into individual parts again.
    This is a sign that perhaps we don’t have the right abstraction yet.

    Another indicator that shows there’s room for improvement is the `config` part of `parse_config`, which implies that the two values we return are related and are both part of one configuration value.

    We’re not currently conveying this meaning in the structure of the data other than by grouping the two values into a tuple; we could put the two values into one struct and give each of the struct fields a meaningful name. Doing so will make it easier for future maintainers of this code to understand how the different values relate to each other and what their purpose is.
    
    */
}

*/


/* Version 2 for parse_config */


/*
 The signature of `parse_config` now indicates that it returns a `Config` value. In the body of `parse_config`, where we used to return string slices that reference `String` values in `args`, we now define `Config` to contain owned `String` values.

The `args` variable in `main` is the owner of the argument values and is only letting the `parse_config` function borrow them, which means we’d violate Rust’s borrowing rules if `Config` tried to take ownership of the values in `args`.

The `args` variable in `main` is the owner of the argument values and is only letting the `parse_config` function borrow them, which means we’d violate Rust’s borrowing rules if `Config` tried to take ownership of the values in `args`.


We could manage the `String` data in a number of different ways, but the easiest, though somewhat inefficient, route is to call the `clone` method on the values.

This will make a full copy of the data for the Config instance to own, which takes more time and memory than storing a reference to the string data.

However, cloning the data also makes our code very straightforward because we don’t have to manage the lifetimes of the references; in this circumstance, giving up a little performance to gain simplicity is a worthwhile trade-off.

*/




struct Config {
    query:String,
    filename:String
}

fn parse_config(args:&[String]) -> Config {

    let query = args[1].clone();
    let filename = args[2].clone();
    Config {query, filename}
}
