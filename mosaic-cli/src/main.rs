use anyhow::Result;
use clap::{Parser, Subcommand};
use std::path::PathBuf;

use crate::{config::{load_config, merge_cli_config}, handlers::{handle_create_session, handle_execute, handle_initialize_root, handle_list_sessions, handle_sign, handle_view_root, handle_view_session}};

mod config;
mod handlers;

#[derive(Debug, Parser)]
#[command(name = "mosaic-cli")]
#[command(about = "Mosaic multisig CLI", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    #[arg(short, long, global = true)]
    config: Option<PathBuf>,

    #[arg(long, global = true)]
    rpc_url: Option<String>,

    #[arg(long, global = true)]
    program_id: Option<String>,
}

#[derive(Debug, Subcommand)]
enum Commands {
    InitializeRoot {
        #[arg(short, long, value_delimiter = ',')]
        operators: Vec<String>,

        #[arg(short, long)]
        threshold: u8,

        #[arg(short, long)]
        destination_program: String,

        #[arg(short, long)]
        payer: Option<PathBuf>,
    },

    CreateSession {
        #[arg(short, long)]
        session_id: u16,

        // (hex string)
        #[arg(short, long)]
        instruction_data: String,

        // Accounts represented as json
        #[arg(short, long)]
        accounts: String,

        #[arg(short, long)]
        payer: Option<PathBuf>,
    },

    Sign {
        #[arg(short, long)]
        session_id: u16,

        #[arg(short, long)]
        signer: PathBuf,
    },

    Execute {
        #[arg(short, long)]
        session_id: u16,

        /// public key
        #[arg(long)]
        storage_account: String,

        #[arg(short, long)]
        executor: PathBuf,
    },

    ViewRoot,

    ViewSession {
        #[arg(short, long)]
        session_id: u16,
    },

    ListSessions,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    let mut config = load_config(cli.config.as_ref())?;
    merge_cli_config(&mut config, &cli);

    match cli.command {
        Commands::InitializeRoot {
            operators,
            threshold,
            destination_program,
            payer,
        } => {
            handle_initialize_root(&config, operators, threshold, destination_program, payer)
                .await?
        }
        Commands::CreateSession {
            session_id,
            instruction_data,
            accounts,
            payer,
        } => handle_create_session(&config, session_id, instruction_data, accounts, payer).await?,
        Commands::Sign { session_id, signer } => handle_sign(&config, session_id, signer).await?,
        Commands::Execute {
            session_id,
            storage_account,
            executor,
        } => handle_execute(&config, session_id, storage_account, executor).await?,
        Commands::ViewRoot => handle_view_root(&config).await?,
        Commands::ViewSession { session_id } => handle_view_session(&config, session_id).await?,
        Commands::ListSessions => handle_list_sessions(&config).await?,
    }

    Ok(())
}
