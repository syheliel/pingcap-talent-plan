extern crate clap;
extern crate structopt;
use std::process::exit;


use structopt::StructOpt;
use std::env::current_dir;

#[derive(Debug, StructOpt)]
#[structopt(name = "git", about = "the stupid content tracker")]
enum Opts {
    #[structopt(name = "set")]
    Set {
        #[structopt()]
        key: String,
        #[structopt()]
        value: String,
    },
    #[structopt(name = "get")]
    Get {
        #[structopt()]
        key: String,
    },
    #[structopt(name = "rm")]
    Remove {
        #[structopt()]
        key: String,
    },
}

fn main() -> std::io::Result<()>{
    let opts = Opts::from_args();
    let mut cur_kvs = kvs::KvStore::open(current_dir()?).expect("wrong when open kvs");
    match opts {
        Opts::Set { key, value } => {
            cur_kvs.set(key,value).expect("error when set key value");
        }
        Opts::Get { key } => {
            if let Some(value) = cur_kvs.get(key).expect("Key not found") {
                println!("{}", value);
            } else {
                println!("Key not found");
            }
        }
        Opts::Remove { key } => {
            match cur_kvs.remove(key) {
                Ok(()) => {}
                Err(_err) => {
                    println!("Key not found");
                    exit(1);
                }
            }
        }
    }
    Ok(())
}
