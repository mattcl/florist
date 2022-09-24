use anyhow::Result;

mod cli;

fn main() -> Result<()> {
    cli::Cli::run()
}
