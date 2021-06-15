use format_serde_error::SerdeError;
use std::io::{
    self,
    Read,
};

mod error_line;

use error_line::{
    parse,
    ErrorLine,
};

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    let mut stdin = io::stdin(); // We get `Stdin` here.
    stdin.read_to_string(&mut buffer)?;

    let errors = parse(&buffer).unwrap();

    for error in errors {
        let input = std::fs::read_to_string(&error.file_path).unwrap();
        println!("{}", SerdeError::new(input, error));
    }

    Ok(())
}
