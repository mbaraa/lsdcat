mod cli;
mod errors;
mod lsd;

fn main() -> Result<(), errors::AppError> {
    let (freq, spread, is_bg) = cli::get_cli_flags()?;
    let color_mode = match is_bg.value() {
        1.0 => lsd::ColorMode::Background,
        _ => lsd::ColorMode::Foreground,
    };

    let result = lsd::rainbowise_stdin(freq.value(), spread.value(), color_mode);
    if result.is_err() {
        return Err(errors::AppError::Io(result.err().unwrap()));
    }

    Ok(())
}
