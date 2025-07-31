use clap::Parser;
use derive_getters::Getters;
use once_cell::sync::Lazy;

pub static ARGS: Lazy<Args> = Lazy::new(Args::parse);

#[derive(Parser, Debug, Getters)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(short, long, default_value_t = false)]
    test_user: bool,
    #[arg(short, long, default_value_t = false)]
    generate_salt: bool,
    #[arg(short, long, default_value_t = false)]
    no_salt: bool,
    #[arg(short, long, default_value_t = false)]
    no_spf: bool,
    #[arg(short, long, default_value_t = false)]
    no_rate_limit: bool,
}
