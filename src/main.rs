use std::{path::Path, error::Error};

use clap::Parser;
use rustfuck::{Rustfuck, RFArgs};

mod rustfuck;

fn main() -> Result<(), Box<dyn Error>> {
    let args = RFArgs::parse();

    let path = Path::new(&args.path);

    if !path.is_file() {
        println!("{} is not a file.", args.path);

        std::process::exit(1);
    }

    Rustfuck::new(&args.path).interp()?;

    Ok(())
}