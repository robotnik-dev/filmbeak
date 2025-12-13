use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use anyhow::Result;
use rand::{rng, seq::IndexedRandom};

#[derive(Debug)]
struct Config {
    films: Vec<String>,
}

fn main() -> Result<()> {
    let config = read_into_config("films.txt")?;

    let random = config
        .films
        .choose(&mut rng())
        .expect("succesfully pick a random movie");

    println!("{}", random);

    Ok(())
}

fn read_into_config(path: &str) -> Result<Config> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut films = vec![];
    for film in reader.lines().map(|l| l.unwrap()) {
        films.push(film);
    }

    Ok(Config { films })
}
