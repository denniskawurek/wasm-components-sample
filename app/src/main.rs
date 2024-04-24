#[allow(warnings)]
mod bindings;

use crate::bindings::dkwr::stringoperations::len::len;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        println!("Usage: {} <string>", args[0]);
        return;
    }

    let input_string = &args[1];

    let result = len(input_string);
    println!("{}", result);
}
