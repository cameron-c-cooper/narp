extern crate simplelog;

use narp::{self, init_term_logging};

fn main() {
    init();
}

fn init() {
    init_term_logging();
}
