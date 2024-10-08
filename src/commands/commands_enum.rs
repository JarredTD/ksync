use clap::Subcommand;

#[derive(Subcommand, Debug)]
pub enum Commands {
    Init {},
    New { bucket: String },
    Config { key: String, value: String },
}
