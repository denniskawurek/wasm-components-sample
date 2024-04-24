#[allow(warnings)]
mod bindings;

use crate::bindings::exports::dkwr::stringlength::len::Guest;

struct Component;

impl Guest for Component {
    /// Say hello!
    fn len(s: String) -> u32 {
        s.len().try_into().unwrap()
    }
}

bindings::export!(Component with_types_in bindings);
