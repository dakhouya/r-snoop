use anyhow::Result;
use clap::Parser;

use r_snoop::ui::App;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    interface: String,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let mut app = App::new(args.interface);
    app.run()?;

    Ok(())
}
