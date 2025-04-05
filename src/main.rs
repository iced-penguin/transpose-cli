use clap::Parser;
use std::io::{self, BufRead};
use std::process;

#[derive(Parser, Debug)]
#[command(about)]
struct Args {
    /// Field delimiter
    #[arg(short, long, default_value = ",")]
    delimiter: String,
}

fn main() {
    let args = Args::parse();

    let delimiter = &args.delimiter;

    let stdin = io::stdin();
    let input: Vec<Vec<String>> = stdin
        .lock()
        .lines()
        .filter_map(Result::ok)
        .map(|line| line.split(delimiter).map(String::from).collect())
        .collect();

    if input.is_empty() {
        return;
    }

    let transposed = match transpose(&input) {
        Ok(result) => result,
        Err(err) => {
            eprintln!("{}", err);
            process::exit(1);
        }
    };

    for row in transposed {
        println!("{}", row.join(delimiter));
    }
}

fn transpose(input: &[Vec<String>]) -> Result<Vec<Vec<String>>, String> {
    let row_length = input[0].len();
    if !input.iter().all(|row| row.len() == row_length) {
        return Err("Error: All rows must have the same number of elements.".to_string());
    }

    let mut result = vec![vec![]; row_length];
    for row in input {
        for (i, value) in row.iter().enumerate() {
            result[i].push(value.clone());
        }
    }
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transpose_multiple_rows_and_columns() {
        let input = vec![
            vec!["a".to_string(), "b".to_string(), "c".to_string()],
            vec!["d".to_string(), "e".to_string(), "f".to_string()],
        ];
        let expected = vec![
            vec!["a".to_string(), "d".to_string()],
            vec!["b".to_string(), "e".to_string()],
            vec!["c".to_string(), "f".to_string()],
        ];
        assert_eq!(transpose(&input).unwrap(), expected);
    }

    #[test]
    fn test_transpose_error_on_unequal_row_lengths() {
        let input = vec![
            vec!["a".to_string(), "b".to_string()],
            vec!["c".to_string()],
        ];
        assert!(transpose(&input).is_err());
    }
}
