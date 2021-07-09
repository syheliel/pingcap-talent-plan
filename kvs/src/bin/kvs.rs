extern crate clap;


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

    if let Some(matches) = matches.subcommand_matches("set"){
        let key = matches.value_of("KEY").unwrap();
        let value = matches.value_of("VALUE").unwrap();
        eprintln!("unimplemented");
    }

    if let Some(matches) = matches.subcommand_matches("get"){
        let key = matches.value_of("KEY").unwrap();
        eprintln!("unimplemented");
    }

    if let Some(matches) = matches.subcommand_matches("rm"){
        let key = matches.value_of("KEY").unwrap();
        eprintln!("unimplemented");
    }

    

}
