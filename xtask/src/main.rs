use std::{fmt::Debug, fs::File, io::Read, path::PathBuf};

use clap::Parser;
use eyre::WrapErr;

mod generate {
    pub mod codegen;
    pub mod config;
    pub mod schema;
    pub mod target;
    pub mod translate;
}

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
        output: PathBuf,
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
            output,
            bless,
        } => {
            let meta_model = File::open(meta_model).wrap_err("could not open meta model")?;
            let meta_model = serde_json::from_reader::<_, generate::schema::MetaModel>(meta_model)
                .wrap_err("could not deserialize meta model")?;

            let mut config_buf = Vec::new();
            _ = File::open(config)
                .wrap_err("could not open config")?
                .read_to_end(&mut config_buf)?;
            let config = toml::from_slice::<generate::config::Config>(&config_buf)
                .wrap_err("could not deserialize config")?;

            let schema = generate::translate::translate_schema(&meta_model, &config)
                .wrap_err("could not translate schema")?;

            let mut output =
                File::create(output).wrap_err("could not open generated output file")?;
            generate::codegen::codegen_schema(&mut output, &schema)?;
        }
    }

    Ok(())
}
