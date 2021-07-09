extern crate clap;
extern crate structopt;
use std::process::exit;

use clap::{crate_authors, crate_name, crate_version, App, Arg, SubCommand};
use structopt::StructOpt;
#[derive(Debug, StructOpt)]
#[structopt(name = "git", about = "the stupid content tracker")]
enum Opts {
    #[structopt(name = "set")]
    Add {
        #[structopt()]
        key: String,
        #[structopt()]
        value: String,
    },
    #[structopt(name = "get")]
    Fetch {
        #[structopt()]
        key: String,
    },
    #[structopt(name = "rm")]
    Commit {
        #[structopt()]
        key: String,
    },
}

fn main() {
    let opts = Opts::from_args();
    match opts {
        Opts::Add { key, value } => {
            eprintln!("unimplemented");
            exit(1)
        }
        Opts::Fetch { key } => {
            eprintln!("unimplemented");
            exit(1)
        }
        Opts::Commit { key } => {
            eprintln!("unimplemented");
            exit(1)
        }
        _ => {
            unreachable!()
        }
    }
}
