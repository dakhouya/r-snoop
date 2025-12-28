use anyhow::Result;
use clap::Parser;

use r_snoop::capture::Sniffer;

#[derive(Parser, Debug)]
#[command(author, version, about = "Passive Asset Fingerprinter")]
struct Args {
    #[arg(short, long)]
    interface: String,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let sniffer = Sniffer::new(&args.interface);
    sniffer.run()?;

    Ok(())
}
