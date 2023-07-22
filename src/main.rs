mod database;
mod email;
mod user;
mod cli;

use std::str;
use cli::MailCLI;
use clap::Parser;
const SLED_PATH: &str = "~/.config/mailrs/mail";

fn main() -> anyhow::Result<()> {
    let tree = sled::open(SLED_PATH).unwrap();
    let db = database::database::new(tree);

    let mail_cli=MailCLI::parse();
    match mail_cli.cmd{
        cli::cmd::User { user }=>{
            match user{
                cli::User::Add { fname, lname, display_name, email, provider, password }=>{
                    let id=display_name.clone();
                    let u=user::User::new(fname, lname, display_name, email, email::get_email_provider(provider.as_str()),
                     password);
                    db.insert_user(u)?;
                    db.make_default_user(id.as_str())?;
                },
                cli::User::Delete { id }=>{
                    println!("delete the sled db file itself -> {}, id {} delete not implemented yet",SLED_PATH,id);
                },
            }
        }
        cli::cmd::Mail{content_file, to_file}=>send_mail(db, content_file, to_file)?,
    }
    
    
    
    Ok(())
}


fn send_mail(db: database::database, content_file: String, to_file: String)-> anyhow::Result<()>{
    let default_user_id = opt_to_res(db.get_default_user_id()?,"No default user id found, please cretae a default user first".to_string())?;
    let default_user=opt_to_res(db.get_user(str::from_utf8(&default_user_id).unwrap())?,"No default user found, please cretae a default user first".to_string())?;
    let u=user::User::from_str(str::from_utf8(&default_user).unwrap().to_string());
    let mailer=email::emailer::new(u.email_provider,u);
    let content=std::fs::read_to_string(content_file)?;
    let subject=content.lines().next().unwrap();
    let body=content.lines().skip(1).collect::<Vec<&str>>().join("\n");

    let tos=std::fs::read_to_string(to_file)?;
    for to in tos.lines(){
        mailer.send_mail(body.as_str(),subject,to)?;
    }
    Ok(())
}

fn opt_to_res<T>(opt: Option<T>,err_msg: String) -> anyhow::Result<T>{
    match opt{
        Some(t)=>Ok(t),
        None=>anyhow::bail!(err_msg),
    }
}