mod opts;
mod process;

pub use opts::{Opts, SubCommand, GenPassOpts};
pub use process::{process_csv, process_genpass};