#![allow(unused)] //TODO: remove on implementation

use std::path::PathBuf;

use anyhow::Result;

use crate::config::Config;


pub async fn handle_initialize_root(
    config: &Config,
    operators: Vec<String>,
    threshold: u8,
    destination_program: String,
    payer: Option<PathBuf>,
) -> Result<()> {
    println!("Initializing root account...");
    println!("Operators: {:?}", operators);
    println!("Threshold: {}", threshold);
    println!("Destination program: {}", destination_program);
    
    todo!("Initialize root implementation")
}

pub async fn handle_create_session(
    config: &Config,
    session_id: u16,
    instruction_data: String,
    accounts: String,
    payer: Option<PathBuf>,
) -> Result<()> {
    println!("Creating signing session {}...", session_id);
    
    todo!("Create session implementation")
}

pub async fn handle_sign(
    config: &Config,
    session_id: u16,
    signer: PathBuf,
) -> Result<()> {
    println!("Signing session {}...", session_id);
    
    todo!("Sign implementation")
}

pub async fn handle_execute(
    config: &Config,
    session_id: u16,
    storage_account: String,
    executor: PathBuf,
) -> Result<()> {
    println!("Executing session {}...", session_id);
    
    todo!("Execute implementation")
}

pub async fn handle_view_root(config: &Config) -> Result<()> {
    println!("Fetching root account state...");
    
    todo!("View root implementation")
}

pub async fn handle_view_session(
    config: &Config,
    session_id: u16,
) -> Result<()> {
    println!("Fetching signing session {}...", session_id);
    
    todo!("View session implementation")
}

pub async fn handle_list_sessions(config: &Config) -> Result<()> {
    println!("Listing all signing sessions...");

    
    todo!("List sessions implementation")
}
