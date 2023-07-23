use clap::Parser;

#[derive(Debug, Parser)]
#[command(name = "mail", author, version, about, long_about)]
pub struct MailCLI {
    #[clap(subcommand)]
    pub cmd: Cmd,
}

#[derive(Debug, Parser)]
pub enum Cmd {
    User {
        #[clap(subcommand)]
        user: User,
    },
    Mail {
        #[clap(long, short)]
        content_file: String,
        #[clap(long, short)]
        to_file: String,
        #[clap(long, short)]
        dry_run: bool,
        #[clap(long, short)]
        send: bool,
    },
    Reset
}

#[derive(Debug, Parser)]
pub enum User {
    Add {
        #[clap(long, short)]
        fname: String,
        #[clap(long, short)]
        lname: String,
        #[clap(long, short)]
        display_name: String,
        #[clap(long, short)]
        email: String,
        #[clap(long, short)]
        provider: String,
        #[clap(long, short)]
        password: String,
    },
    List,
    Default,
    Delete {
        #[clap(long, short)]
        id: String,
    },
}
