use clap::Parser;
use rcli::{Opts, process_csv, SubCommand};

fn main() -> anyhow::Result<()> {
    let opt = Opts::parse();
    match opt.cmd {
        SubCommand::Csv(opts) => {
            let output = if let Some(output) = opts.output {
                output.clone()
            } else {
                format!("output.{}", opts.format)
            };
            process_csv(&opts.input, output, opts.format)?;
        }
    }
    Ok(())
}