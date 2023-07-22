mod database;
mod email;
mod user;
mod cli;

use std::str;
use cli::MailCLI;
use clap::Parser;
const SLED_PATH: &str = "~/.config/mailrs/mail";

fn main() -> anyhow::Result<()> {
    let a=MailCLI::parse();
    let tree = sled::open(SLED_PATH).unwrap();
    let db = database::database::new(tree);
    let default_user_id = opt_to_res(db.get_default_user_id()?,"No default user id found, please cretae a default user first".to_string())?;
    let default_user=opt_to_res(db.get_user(str::from_utf8(&default_user_id).unwrap())?,"No default user found, please cretae a default user first".to_string())?;
    
    Ok(())
}


fn opt_to_res<T>(opt: Option<T>,err_msg: String) -> anyhow::Result<T>{
    match opt{
        Some(t)=>Ok(t),
        None=>anyhow::bail!(err_msg),
    }
}