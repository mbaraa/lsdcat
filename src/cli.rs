use std::collections::HashMap;
use std::env;

use crate::errors;

pub fn get_cli_flags() -> Result<(CliFlag, CliFlag, CliFlag), errors::AppError> {
    let args: Vec<String> = env::args().collect();
    let mut args_flat = HashMap::<&str, String>::from([
        ("--freq", "0.1".to_string()),
        ("--spread", "3.0".to_string()),
        ("--bg", "0.0".to_string()),
    ]);

    let mut i = 1;
    while i < args.len() {
        if i + 2 > args.len() {
            break;
        }
        args_flat.insert(&args[i], args[i + 1].clone());
        i += 2;
    }

    Ok((
        CliFlag::new(
            "--freq".to_string().clone(),
            args_flat.get("--freq").unwrap().clone(),
        )?,
        CliFlag::new(
            "--spread".to_string().clone(),
            args_flat.get("--spread").unwrap().clone(),
        )?,
        CliFlag::new(
            "--bg".to_string().clone(),
            args_flat.get("--bg").unwrap().clone(),
        )?,
    ))
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub enum CliFlag {
    Freq(i32),
    Spread(i32),
    ColorMode(i32),
}

impl CliFlag {
    pub fn new(flag_name: String, val: String) -> Result<Self, errors::AppError> {
        let num_val_result = val.parse::<f32>();
        if num_val_result.is_err() {
            return Err(errors::AppError::InvalidArgumentValue);
        }
        let num_val = num_val_result.unwrap();

        match flag_name.as_str() {
            "--freq" => Ok(Self::Freq((num_val * 1000.0) as i32)),
            "--spread" => Ok(Self::Spread((num_val * 1000.0) as i32)),
            "--bg" => Ok(Self::ColorMode((num_val * 1000.0) as i32)),
            _ => Err(errors::AppError::InvalidArgumentName),
        }
    }

    pub fn value(&self) -> f32 {
        match self {
            Self::Freq(f) => *f as f32 / 1000.0,
            Self::Spread(s) => *s as f32 / 1000.0,
            Self::ColorMode(cm) => *cm as f32 / 1000.0,
        }
    }
}
