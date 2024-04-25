use clap::Parser;
use rcli::{Opts, process_csv, Subcommand};

fn main() -> anyhow::Result<()> {
    let opt = Opts::parse();
    match opt.cmd {
        Subcommand::Csv(opts) => process_csv(&opts.input, &opts.output)?,
    }
    Ok(())
}