


use rcli::*;
use clap::Parser;


use anyhow;


//  cargo run -- csv  -i debug.csv
fn main() -> anyhow::Result<()> {
    println!("Hello, world!");
    let opts:Opts = Opts::parse();
    match opts.cmd {
        SubCommands::Csv(csv_opts) => process_csv(&csv_opts.input,&csv_opts.output)
    }

}
