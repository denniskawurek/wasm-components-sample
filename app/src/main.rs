#[allow(warnings)]
mod bindings;

use crate::bindings::dkwr::stringoperations::len::len;

fn main() {
    let args: Vec<String> = std::env::args().collect(); // Collect command line arguments into a vector of strings
    if args.len() != 2 {
        println!("Usage: {} <string>", args[0]);
        return;
    }

    let input_string = &args[1]; // Extract the input string from command line arguments

    let result = len(input_string);
    println!("{}", result);
}
