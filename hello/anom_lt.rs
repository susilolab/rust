#![allow(unused_variables)]
use std::fmt;

struct StrWrap<'a>(&'a str);

fn make_wrapper(string: &str) -> StrWrap<'_> {
    StrWrap(string)
}

impl fmt::Debug for StrWrap<'_> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.write_str(self.0)
    }
}

fn main() {}
