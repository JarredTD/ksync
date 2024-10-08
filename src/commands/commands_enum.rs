use clap::Subcommand;

#[derive(Subcommand, Debug)]
pub enum Commands {
    #[command(about = "Initialize a new project.")]
    Init {},

    #[command(about = "Create a new bucket with the specified name.")]
    New {
        #[arg(help = "The name of the bucket to create.")]
        bucket: String,
    },

    #[command(about = "Set a key-value pair in the configuration.")]
    Config {
        #[arg(help = "The configuration key to set.")]
        key: String,

        #[arg(help = "The value to set for the configuration key.")]
        value: String,
    },
}
