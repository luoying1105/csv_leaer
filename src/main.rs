


use rcli::*;
use clap::Parser;


use anyhow;


//  cargo run -- csv  -i debug.csv
fn main() -> anyhow::Result<()> {
    println!("Hello, world!");
    let opts:Opts = Opts::parse();
    match opts.cmd {
        // 如果output有值就用,没有给个默认值

        SubCommands::Csv(csv_opts) => {
            let output = if let Some(output) = csv_opts.output {
                    output.clone()
                } else {
                    format!("output.{}",csv_opts.format)
                };
                process_csv(&csv_opts.input, output, csv_opts.format)?;
          

            // Ok(process_csv(&csv_opts.input, output, csv_opts.format)?)
        }
        SubCommands::GenPass(gen_opts) => {
            process_genpass(gen_opts.length,gen_opts.uppercase,gen_opts.lowercase,
                            gen_opts.numbers,gen_opts.symbols);

        }
    }
    Ok(())
}
