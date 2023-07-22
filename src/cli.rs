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
        content_file: String,
        #[clap(long,short)]
        to_file: String,
    },
}

#[derive(Debug, Parser)]
pub enum User{
    Add{
        #[clap(long,short)]
        fname: String,
        #[clap(long,short)]
        lname: String,
        #[clap(long,short)]
        display_name: String,
        #[clap(long,short)]
        email: String,
        #[clap(long,short)]
        provider: String,
        #[clap(long,short)]
        password: String,
    },
    Delete{
        #[clap(long,short)]
        id: String,
    },
}