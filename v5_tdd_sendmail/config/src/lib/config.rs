use clap::Args;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use anyhow::{Result, Context, bail};
#[derive(Debug, Clone, Serialize, Deserialize, Args)]
pub struct Config {
  #[arg(long, short='f')]
  pub email_from: Option<String>,
  #[arg(long, short='r')]
  pub email_reply: Option<String>,
  #[arg(long, short='t')]
  pub email_to: Option<String>,
  #[arg(long, short='u')]
  pub credentials_usr: Option<String>,
  #[arg(long, short='p')]
  pub credentials_psw: Option<String>,
  #[arg(long, short='s')]
  pub smtp_server: Option<String>
  #[arg(long, short='c', default_value=".env")]
  pub config_file: Option<String>,
}

impl Config{
    pub fn load(){
        
    }
}