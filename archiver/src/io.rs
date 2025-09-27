use anyhow::{Context, Result};
use std::{env, path::PathBuf, str::FromStr};

use crate::Codes;

mod choice_encoder;
pub use choice_encoder::EncoderChoice;

pub fn print_codes(name: &str, codes: &Codes) {
    println!("{}:", name);
    println!("Probabilities: {:?}", codes.probabilities());
    println!("Codes: {:?}", codes.codes());
    println!("Mean length: {}", codes.mean_code_length());
    println!(
        "Relative efficiency ratio: {}",
        codes.relative_efficiency_ratio()
    );
    println!(
        "Statistical compression ratio: {}",
        codes.statistical_compression_ratio()
    );
    print!("\n");
}

pub fn read_vec_numbers<T>(output: &str) -> Vec<T>
where
    T: FromStr,
    <T as FromStr>::Err: std::fmt::Debug,
{
    let stdin = std::io::stdin();
    let mut buf = String::new();

    println!("\n{}", output);

    stdin.read_line(&mut buf).unwrap();
    return buf
        .trim()
        .split_whitespace()
        .map(|s| s.parse::<_>().unwrap())
        .collect::<Vec<_>>();
}

pub fn read_filepath(output: &str) -> Result<PathBuf> {
    let stdin = std::io::stdin();
    let mut buf = String::new();

    println!("\n{}", output);
    stdin.read_line(&mut buf).context("Failed to read input")?;

    let path_str = buf.trim();
    if path_str.is_empty() {
        anyhow::bail!("No path provided");
    }

    let path = PathBuf::from(path_str);
    path_to_absolute(path)
}

pub fn path_to_absolute(path: PathBuf) -> Result<PathBuf> {
    if path.is_absolute() {
        Ok(path)
    } else {
        let current_dir = env::current_dir().context("Failed to get current directory")?;
        let absolute_path = current_dir.join(&path);

        // Пытаемся канонизировать, но если файл не существует, это нормально
        match absolute_path.canonicalize() {
            Ok(canonical) => Ok(canonical),
            Err(_) => Ok(absolute_path),
        }
    }
}
