use std::path::PathBuf;

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(about, author)]
pub struct Opt {
    #[structopt(
        long,
        help = "Check application functioning (without running it actually)"
    )]
    pub check: bool,
    #[structopt(short, long, help = "Path to configuration", env = "OTUS__CONFIG_FILE")]
    pub config: Option<PathBuf>,
}
