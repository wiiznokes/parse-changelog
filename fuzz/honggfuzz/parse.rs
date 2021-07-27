use std::str;

use honggfuzz::fuzz;
use parse_changelog::parse;

fn main() {
    loop {
        fuzz!(|string: &str| {
            let _result = parse(string);
        });
    }
}
