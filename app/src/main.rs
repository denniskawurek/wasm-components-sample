#[allow(warnings)]
mod bindings;

use crate::bindings::dkwr::stringlength::len::len;

fn main() {
    let result = len("d");
    println!("{result}");
}
