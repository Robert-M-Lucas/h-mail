use clap::Parser;
use derive_getters::Getters;
use once_cell::sync::Lazy;

pub static ARGS: Lazy<Args> = Lazy::new(Args::parse);

#[derive(Parser, Debug, Getters)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(short, long)]
    port: u16,
}
