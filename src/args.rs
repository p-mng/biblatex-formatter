use clap::Parser;

#[derive(Debug, Parser)]
pub struct Args {
    /// Generate bibtex instead of biblatex
    #[arg(short, long)]
    pub bibtex: bool,
    /// Output filename (leave empty to print to stdout)
    #[arg(short, long)]
    pub output_filename: Option<String>,
    /// Edit input filename in-place
    #[arg(short, long)]
    pub inplace: bool,
    /// Input filename (leave empty to read from stdin)
    pub filename: Option<String>,
}
