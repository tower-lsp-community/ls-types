use std::{
    fs::File,
    io::{self, Read, Write},
    path::PathBuf,
};

use clap::Parser;
use eyre::WrapErr;

mod codegen;
mod config;
mod schema;
mod target;
mod translate;

#[derive(clap::Parser)]
struct Args {
    #[clap(subcommand)]
    cmd: Command,
}

#[derive(Debug, Clone, clap::Subcommand)]
enum Command {
    Generate {
        meta_model: PathBuf,
        #[clap(long)]
        config: PathBuf,
        #[clap(long)]
        bless: bool,
    },
}

fn main() -> eyre::Result<()> {
    let args = Args::parse();

    match args.cmd {
        Command::Generate {
            meta_model,
            config,
            bless,
        } => {
            let meta_model = File::open(meta_model).wrap_err("could not open meta model")?;
            let meta_model = serde_json::from_reader::<_, schema::MetaModel>(meta_model)
                .wrap_err("could not deserialize meta model")?;

            let mut config_buf = Vec::new();
            _ = File::open(config)
                .wrap_err("could not open config")?
                .read_to_end(&mut config_buf)?;
            let config = toml::from_slice::<config::Config>(&config_buf)
                .wrap_err("could not deserialize config")?;

            let items = translate::translate_schema(&meta_model, &config)
                .wrap_err("could not translate schema")?;
        }
    }

    Ok(())
}
