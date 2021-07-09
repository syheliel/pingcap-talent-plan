extern crate clap;


use std::process::exit;

use clap::{App, Arg, SubCommand, crate_authors, crate_name, crate_version};
fn main() {
    let matches = App::new(crate_name!())
    .author(crate_authors!())
        .version(crate_version!())
        .subcommand(
            SubCommand::with_name("set")
            .args(
                &[Arg::with_name("KEY").required(true),
                Arg::with_name("VALUE").required(true)]
            )
        )
    .subcommand(
        SubCommand::with_name("get")
        .arg(Arg::with_name("KEY").required(true))
    )
    .subcommand(
        SubCommand::with_name("rm")
        .arg(Arg::with_name("KEY").required(true))
    )
    .get_matches();

    match matches.subcommand(){
        ("set",Some(_matches)) => {
            eprintln!("unimplemented");
            exit(1);
        }
        ("get",Some(_matches)) => {
            eprintln!("unimplemented");
            exit(1);
        }
        ("rm",Some(_matches)) => {
            eprintln!("unimplemented");
            exit(1);
        }
        _ => unreachable!()
    }


}
