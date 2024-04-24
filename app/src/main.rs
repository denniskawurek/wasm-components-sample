#[allow(warnings)]
mod bindings;

use crate::bindings::dkwr::stringoperations::len::len;

fn main() {
    let result = len("d");
    println!("{result}");
}
