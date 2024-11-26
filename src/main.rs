use std::io::Read;

use anyhow::{anyhow, Context, Result};
use clap::Parser;

mod args;

fn main() -> Result<()> {
    let args = args::Args::parse();

    let content = match args.filename {
        Some(ref filename) => std::fs::read_to_string(filename)?,
        None => {
            let mut buf = String::new();
            std::io::stdin().read_to_string(&mut buf)?;
            buf
        }
    };

    let bibliography = biblatex::Bibliography::parse(&content)
        .map_err(|err| anyhow!("error parsing bibliography: {}", err))?;

    let formatted = match args.bibtex {
        true => bibliography.to_bibtex_string(),
        false => bibliography.to_biblatex_string(),
    };

    let mut output_file: Box<dyn std::io::Write> = if args.inplace {
        let filename = &args
            .filename
            .context("cannot use --inplace when reading from stdin")?;
        let file = std::fs::File::create(filename)?;
        Box::from(file)
    } else if args.output_filename.is_some() {
        let file = std::fs::File::create(args.output_filename.unwrap())?;
        Box::from(file)
    } else {
        Box::from(std::io::stdout())
    };

    output_file.write_all(formatted.as_bytes())?;

    Ok(())
}
