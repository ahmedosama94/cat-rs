use cat::CatArgs;
use clap::Parser;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = CatArgs::parse();

    print!("{}", args.exec()?);

    Ok(())
}
