mod cli;
mod database;
mod email;
mod user;

use clap::Parser;
use cli::MailCLI;
use handlebars::{to_json, Handlebars};
use serde_json::value::Map;
use std::str;

const SLED_PATH: &str = "~/.config/mailrs/mail";

fn main() -> anyhow::Result<()> {
    let tree = sled::open(SLED_PATH).unwrap();
    let db = database::Database::new(tree);

    let mail_cli = MailCLI::parse();
    match mail_cli.cmd {
        cli::Cmd::Reset => {
            db.clear()?;
        }
        cli::Cmd::User { user } => match user {
            cli::User::List => {
                let users = db.get_users()?;
                for user in users {
                    println!("{}", str::from_utf8(&user).unwrap());
                }
            }
            cli::User::Default => {
                let user = db.get_default_user_id()?;
                println!("{}", str::from_utf8(&user.unwrap()).unwrap());
            }
            cli::User::Add {
                fname,
                lname,
                display_name,
                email,
                provider,
                password,
            } => {
                let id = display_name.clone();
                let u = user::User::new(
                    fname,
                    lname,
                    display_name,
                    email,
                    email::get_email_provider(provider.as_str()),
                    password,
                );
                db.insert_user(u)?;
                db.make_default_user(id.as_str())?;
            }
            cli::User::Delete { id } => {
                println!(
                    "delete the sled db file itself -> {}, id {} delete not implemented yet",
                    SLED_PATH, id
                );
            }
        },
        cli::Cmd::Mail {
            content_file,
            to_file,
            dry_run,
            send,
        } => send_mail(db, content_file, to_file, dry_run, send)?,
    }

    Ok(())
}

fn send_mail(
    db: database::Database,
    content_file: String,
    to_file: String,
    dry_run: bool,
    send: bool,
) -> anyhow::Result<()> {
    let default_user_id = opt_to_res(
        db.get_default_user_id()?,
        "No default user id found, please create a default user first".to_string(),
    )?;
    let default_user = opt_to_res(
        db.get_user(str::from_utf8(&default_user_id).unwrap())?,
        "No default user found, please create a default user first".to_string(),
    )?;
    let u = user::User::from_str(str::from_utf8(&default_user).unwrap().to_string());
    let mailer = email::Emailer::new(u.email_provider, u);
    let mut handlebars = Handlebars::new();
    handlebars.register_template_file("email", content_file)?;

    let tos = std::fs::read_to_string(to_file)?;
    for to in tos.lines() {
        // TODO: can be optimised to use same memory always, clearing the keys without having to call alloc again and again.
        let mut data = Map::new();
        let to = to.split(',').collect::<Vec<&str>>();
        if to.len() == 2 {
            let args = to[1].trim_matches('"');
            let args = args.split(',').collect::<Vec<&str>>();
            for arg in args {
                let arg = arg.split('=').collect::<Vec<&str>>();
                data.insert(arg[0].to_string(), to_json(arg[1].to_string()));
            }
        }
        let content = handlebars.render("email", &data)?;
        let subject = content.lines().next().unwrap();
        let body = content.lines().skip(1).collect::<Vec<&str>>().join("\n");
        if dry_run {
            println!("subject: {}\nbody: {}\nto: {}", subject, body, to[0]);
        }
        if send {
            mailer.send_mail(body.as_str(), subject, to[0])?;
        }
    }
    Ok(())
}

fn opt_to_res<T>(opt: Option<T>, err_msg: String) -> anyhow::Result<T> {
    match opt {
        Some(t) => Ok(t),
        None => anyhow::bail!(err_msg),
    }
}
