use clap::Parser;
use rcli::{process_csv, Opts, SubCommand};

// rcli csv -i input.csv -o output.json --header -d ','
fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();
    println!("{:?}", opts);
    match opts.cmd {
        SubCommand::Csv(opts) => {
            let output: String = if let Some(output) = opts.output {
                output.clone()
            } else {
                format!("output.{}", opts.format)
            };
            process_csv(&opts.input, output, opts.format)?;
        }
    }
    Ok(())
}
