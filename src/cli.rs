use clap::Parser;

#[derive(Debug, Parser)]
#[command(name = "mail", author, version, about, long_about)]
pub struct MailCLI {
    #[clap(subcommand)]
    pub cmd: cmd,
}


#[derive(Debug, Parser)]
pub enum cmd{
    User{
        #[clap(subcommand)]
        user: User,
    },
    Mail{
        #[clap(long,short)]
        provider: String,
        #[clap(long,short)]
        file: String,
    },
}

#[derive(Debug, Parser)]
pub enum User{
    Add,
    Delete,
    Modify,
}