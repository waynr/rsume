use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;

use serde::Deserialize;
use serde::Serialize;

use super::errors::Error;

pub const DEFAULT: &'static str = include_str!("../themes/default.yaml");

#[derive(Serialize, Deserialize)]
pub struct Theme {
    font: String,
    background: String,
    foreground: String,
    h1: String,
    h2: String,
    links: String,
}

impl TryFrom<&PathBuf> for Theme {
    type Error = Error;

    fn try_from(pb: &PathBuf) -> Result<Self, Error> {
        let mut f = File::open(pb)?;
        let mut s = String::new();
        f.read_to_string(&mut s)?;
        Ok(serde_yaml::from_str(s.as_str())?)
    }
}
