use core::panic;
use std::{
    f32::consts,
    io::{stdin, stdout, IsTerminal, Read, Result, Write},
};

fn main() -> Result<()> {
    let mut stdin = stdin();
    let mut buf = String::new();
    stdin.read_to_string(&mut buf)?;

    let mut stdout = stdout();
    if !stdout.is_terminal() {
        panic!("can't do that");
    }

    stdout.write_fmt(format_args!("{}", rainbowise_fg(&buf, 0.1, 3.0)))?;
    stdout.write_fmt(format_args!("{}", rainbowise_bg(&buf, 0.1, 3.0)))?;

    Ok(())
}

fn rainbowise_fg(input: &String, freq: f32, spread: f32) -> String {
    rainbowise(input, freq, spread, ColorMode::Foreground)
}

fn rainbowise_bg(input: &String, freq: f32, spread: f32) -> String {
    rainbowise(input, freq, spread, ColorMode::Background)
}

enum ColorMode {
    Foreground,
    Background,
}

impl ColorMode {
    fn ascii(&self) -> &str {
        match self {
            Self::Foreground => "38",
            Self::Background => "48",
        }
    }
}

fn rainbowise(input: &String, freq: f32, spread: f32, mode: ColorMode) -> String {
    input
        .bytes()
        .into_iter()
        .enumerate()
        .map(|(i, chr)| {
            let (red, green, blue) = taste(freq, i as f32 / spread);
            format!(
                "\x1b[{};2;{};{};{}m{}",
                mode.ascii(),
                red,
                green,
                blue,
                chr as char
            )
        })
        .collect::<Vec<String>>()
        .join("")
        + "\x1b[0m"
}

// inspired from https://github.com/busyloop/lolcat/blob/f4cca5601ea57df2b5b3c98feea8ad05f4421039/lib/lolcat/lol.rb#L36
fn taste(freq: f32, i: f32) -> (u8, u8, u8) {
    let red = ((freq * i).sin() * 127.0 + 128.0) as u8;
    let green = ((freq * i + 2.0 * consts::PI / 3.0).sin() * 127.0 + 128.0) as u8;
    let blue = ((freq * i + 4.0 * consts::PI / 3.0).sin() * 127.0 + 128.0) as u8;

    (red, green, blue)
}
