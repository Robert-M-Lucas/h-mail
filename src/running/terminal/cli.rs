use clap::Parser;

#[derive(Parser, Debug)]
pub enum Cli {
    Send,
    Exit,
}
