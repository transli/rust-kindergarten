use anyhow::{anyhow, Context};
use std::io::{BufRead, Write};

fn main() {
    let continue_ = confirm("\nDo you wish to continue? (y/N)", false).context("false");
    if let Ok(true) = continue_ {
        println!("Ok...")
    } else {
        println!("False ...")
    }
}

pub fn confirm(q: &str, default: bool) -> Result<bool, anyhow::Error> {
    write!(std::io::stdout().lock(), "{q} ")?;

    let _ = std::io::stdout().flush();
    let input = read_line()?;
    let r = match &*input.to_lowercase() {
        "y" | "yes" => true,
        "n" | "no" => false,
        "" => default,
        _ => false,
    };

    writeln!(std::io::stdout().lock())?;

    Ok(r)
}

fn read_line() -> Result<String, anyhow::Error> {
    let stdin = std::io::stdin();
    let stdin = stdin.lock();
    let mut lines = stdin.lines();
    let lines = lines.next().transpose()?;
    match lines {
        None => Err(anyhow!("No lines found from stdin")),
        Some(v) => Ok(v),
    }
    .context("Unable to read from stdin for confirmation")
}