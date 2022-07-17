use rand::{
    distributions::{Alphanumeric, Distribution},
    thread_rng,
};
use std::{env::args, error::Error};

fn main() -> Result<(), Box<dyn Error>> {
    println!(
        "{}",
        Alphanumeric
            .sample_iter(&mut thread_rng())
            .take(
                args()
                    .nth(1)
                    .ok_or_else(|| "missing length argument")?
                    .parse::<usize>()?
            )
            .map(char::from)
            .collect::<String>()
    );
    Ok(())
}
