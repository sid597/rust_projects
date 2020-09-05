/*
/* Version 1 for parse_config */

use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let (query, filename) = parse_config(&args);

    // --snip--

    println!("Searching for {}", query);
    println!("In file {}", filename);

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    println!("With text:\n{}", contents);
}

fn parse_config(args: &[String]) -> (&str, &str) {
    let query = &args[1];
    let filename = &args[2];

    (query, filename)
}


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
/*
 The signature of `parse_config` now indicates that it returns a `Config` value. In the body of `parse_config`, where we used to return string slices that reference `String` values in `args`, we now define `Config` to contain owned `String` values.

The `args` variable in `main` is the owner of the argument values and is only letting the `parse_config` function borrow them, which means we’d violate Rust’s borrowing rules if `Config` tried to take ownership of the values in `args`.

The `args` variable in `main` is the owner of the argument values and is only letting the `parse_config` function borrow them, which means we’d violate Rust’s borrowing rules if `Config` tried to take ownership of the values in `args`.


We could manage the `String` data in a number of different ways, but the easiest, though somewhat inefficient, route is to call the `clone` method on the values.

This will make a full copy of the data for the Config instance to own, which takes more time and memory than storing a reference to the string data.

However, cloning the data also makes our code very straightforward because we don’t have to manage the lifetimes of the references; in this circumstance, giving up a little performance to gain simplicity is a worthwhile trade-off.

*/

use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = parse_config(&args);

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    let contents = fs::read_to_string(config.filename)
        .expect("Something went wrong reading the file");

    // --snip--

    println!("With text:\n{}", contents);
}

struct Config {
    query: String,
    filename: String,
}

fn parse_config(args: &[String]) -> Config {
    let query = args[1].clone();
    let filename = args[2].clone();

    Config { query, filename }
}

*/


/*
/* Version 3 */


/* 
So now that the purpose of the `parse_config` function is to create a `Config` instance, we can change `parse_config` from a plain function to a function named `new` that is associated with the `Config` struct. 

Making this change will make the code more idiomatic. We can create instances of types in the standard library, such as `String`, by calling `String::new`. Similarly, by changing `parse_config` into a `new` function associated with `Config`, we’ll be able to create instances of `Config` by calling `Config::new`.


For error handling if the length of args is small :

a call to panic! is more appropriate for a programming problem than a usage problem, so we will use `Result` in this case.
*/


use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args);

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    let contents = fs::read_to_string(config.filename)
        .expect("Something went wrong reading the file");

    println!("With text:\n{}", contents);

    // --snip--
}

// --snip--

struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Config {
        let query = args[1].clone();
        let filename = args[2].clone();

        Config { query, filename }
    }
}


*/


// Version 4

/*

/*
For error handling if the length of args is small :

A call to panic! is more appropriate for a programming problem than a usage problem, so we will use `Result` in this case.

*/


use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args);

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    let contents = fs::read_to_string(config.filename)
        .expect("Something went wrong reading the file");

    println!("With text:\n{}", contents);
}

struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}

To handle the error case and print a user-friendly message, we need to update `main` to handle the `Result` being returned by `Config::new`


*/

// Version 5

/*

/*
 If the Result is an Ok value, this method’s behavior is similar to unwrap: it returns the inner value Ok is wrapping. However, if the value is an Err value, this method calls the code in the closure, which is an anonymous function we define and pass as an argument to unwrap_or_else


 We’ve added a new use line to bring process from the standard library into scope. The code in the closure that will be run in the error case is only two lines: we print the err value and then call process::exit. The process::exit function will stop the program immediately and return the number that was passed as the exit status code. This is similar to the panic!-based handling we used in Listing 12-8, but we no longer get all the extra output. 
*/

use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err|{
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    // --snip--

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    let contents = fs::read_to_string(config.filename)
        .expect("Something went wrong reading the file");

    println!("With text:\n{}", contents);
}

struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}

*/

// Version 6

/*
With the remaining program logic separated into the `run` function, we can improve the error handling, as we did with `Config::new`

Instead of allowing the program to panic by calling `expect`, the run function will return a `Result<T, E>` when something goes wrong. This will let us further consolidate into `main` the logic around handling errors in a user-friendly way.

We use `if let` rather than `unwrap_or_else` to check whether `run` returns an `Err` value and call `process::exit(1)` if it does. The `run` function doesn’t return a value that we want to `unwrap` in the same way that `Config::new` returns the `Config` instance. Because `run` returns `()` in the success case, we only care about detecting an error, so we don’t need `unwrap_or_else` to return the unwrapped value because it would only be `()`.

The bodies of the i`f let` and the `unwrap_or_else` functions are the same in both cases: we print the error and exit.

*/


use std::env;
use std::fs;
use std::process;
use std::error::Error;

// --snip--


fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    if let Err(e) = run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    println!("With text:\n{}", contents);

    Ok(())
}

struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}
