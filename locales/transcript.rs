use serde::Deserialize;
use std::{error::Error, fs::File, io::BufReader, path::Path};

/*
	I know this is so slow but there's nothing can we do for now, i tried
	to use rust_i18n but it was good but it didn't work as i want. 
	This is the temprorary solution for now.
*/

#[derive(Deserialize, Debug)]
struct User {
    fingerprint: String,
    location: String,
}

pub fn gettext<P: AsRef<Path>>(path: P) -> Result<User, Box<dyn Error>> {
    // Open the file in read-only mode with buffer.
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    // Read the JSON contents of the file as an instance of `User`.
    let u = serde_json::from_reader(reader)?;

    // Return the `User`.
    Ok(u)
}
