mod commands;
mod config;
mod constants;
mod util;

use clap::{ArgAction, Parser};
use log::LevelFilter;
use std::env;
use std::io::Write;
use std::process::exit;

use commands::commands_enum::Commands;
use commands::commands_impl::{init, new, none};
use config::config_object::Config;
use constants::CONF_FILE_NAME;
use util::root_search::find_ksync_root;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[command(subcommand)]
    command: Option<Commands>,
    #[arg(short, long, action = ArgAction::Count, global = true)]
    verbose: u8,
}

impl Args {
    fn log_level(&self) -> LevelFilter {
        match self.verbose {
            0 => LevelFilter::Warn,  // Default level (warning)
            1 => LevelFilter::Info,  // -v
            2 => LevelFilter::Debug, // -vv
            _ => LevelFilter::Trace, // -vvv or more
        }
    }
}

fn init_logging(level: log::LevelFilter) {
    env_logger::Builder::new()
        .format(|buf, record| {
            let ts = buf.timestamp(); // Add timestamp
            writeln!(
                buf,
                "{} - {} [{}]: {}",
                ts,
                record.level(),
                record.target(),
                record.args()
            )
        })
        .filter(None, level)
        .init();
}

fn main() {
    let args = Args::parse();
    init_logging(args.log_level());
    log::debug!("{:?}", args);

    let current_dir = match env::current_dir() {
        Ok(current_dir) => current_dir,
        Err(e) => {
            log::error!("error finding working directory: {}", e);
            exit(1)
        }
    };

    if let Some(Commands::Init {}) = args.command {
        init(&current_dir);
    }

    let ksync_root = match find_ksync_root(&current_dir) {
        Ok(ksync_root) => ksync_root,
        Err(_) => {
            println!("ksync is not initialized. please run 'ksync init'");
            exit(0)
        }
    };
    let ksync_config = ksync_root.join(CONF_FILE_NAME);
    let mut cfg = Config::read(&ksync_config);

    match args.command {
        Some(Commands::Init {}) => {}
        Some(Commands::Config { key, value }) => {
            cfg.update(key, value);
            if let Err(e) = cfg.write(&ksync_config) {
                eprintln!("Failed to write updated config: {}", e);
            }
        }
        Some(Commands::New { bucket: _ }) => new(),
        None => none(),
    }
}
