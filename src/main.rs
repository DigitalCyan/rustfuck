use std::env;

use rustfuck::Rustfuck;

mod rustfuck;

fn main() {
    let mut args = env::args();

    if let Some(path) = args.nth(1) {
        let mut rf = Rustfuck::new(path);
        rf.load();
        rf.interp();
    } else {
        println!("USAGE: rustfuck <FILE>");
    }
}
