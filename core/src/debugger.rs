use crate::console::types::address::Address;
use std::collections::HashSet;

#[derive(Debug, Default, Clone)]
pub struct Debugger {
    breakpoints: HashSet<Address>,
}
