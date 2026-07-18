use clap::{Parser, ValueEnum};

#[derive(ValueEnum, Clone, Debug)]
pub enum ArgTypes {
    #[value(name = "init")]
    Init,
    Build,
}

#[derive(Parser)]
pub struct Args {
    pub arg_type: ArgTypes,
    #[arg(long, short, default_value_t = String::from("./"))]
    pub directory: String,
    #[arg(long, default_value_t = false)]
    pub debug: bool,
}

pub fn parse_args() -> Args {
    let args = Args::parse();

    args
}
