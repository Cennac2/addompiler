use clap::{Parser, ValueEnum};

#[derive(ValueEnum, Clone, Debug)]
pub enum ArgTypes {
    ///Initialize a new project
    Init,
    ///Build the Addon
    Build,
    ///Watch for changes and rebuild automatically
    Watch,
}

#[derive(Parser)]
pub struct Args {
    pub arg_type: ArgTypes,
    /// Directory to work in
    #[arg(long, short, default_value_t = String::from("./"), hide_default_value = true)]
    pub directory: String,
    /// Profile to use when building
    #[arg(long, short)]
    pub profile: Option<String>,
    /// Print debug logs
    #[arg(long, default_value_t = false)]
    pub debug: bool,
}

pub fn parse_args() -> Args {
    let args = Args::parse();

    args
}
