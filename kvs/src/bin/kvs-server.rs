extern crate clap;
extern crate structopt;
use std::process::exit;
use clap::arg_enum;

use structopt::StructOpt;
use std::net::SocketAddr;
use std::env::current_dir;
use std::str::FromStr;
use std::io::SeekFrom::End;
use kvs::Result;

const ENGINE_FILE_NAME: &str = "engine.conf";
arg_enum! {
    #[allow(non_camel_case_types)]
    #[derive(Debug,Clone, PartialEq,Eq)]
    enum Engine {
        kvs,
        sled
    }
}

#[derive(Debug, StructOpt)]
#[structopt(name = "", about = "the stupid content tracker")]
struct Opt{
    #[structopt(
    long,
    default_value = "127.0.0.1:4000",
    parse(try_from_str))]
    addr : SocketAddr,
    #[structopt(
    long = "engine",
    help = "Sets the storage engine",
    value_name = "ENGINE-NAME",
    possible_values = &Engine::variants(),
    default_value = "kvs"
    )]
    engine: Engine,
}


fn main() -> std::io::Result<()>{
    let opt:Opt = Opt::from_args();
    println!("{:?}",opt);
    match get_engine(){
        Ok(None) => {set_engine(opt.engine.clone());start_server(opt.addr,opt.engine);},
        Ok(Some(engine)) => {
            if(engine != opt.engine.clone()){
                println!("found previous engine : {}, but your current engine is : {}",
                         engine,opt.engine);
                exit(1);
            }else{
                start_server(opt.addr,opt.engine);
            }
        }
        _ => {}
    }

    Ok(())
}

fn start_server(addr:SocketAddr,engine:Engine){
    loop{

    }
}

fn get_engine() -> Result<Option<Engine>>{
    let file_path = current_dir()?.join(ENGINE_FILE_NAME);
    if !file_path.exists(){
        return Ok(None);
    }
    let engine_name =  std::fs::read_to_string(file_path)?;
    match Engine::from_str(&*engine_name) {
        Ok(engine) => Ok(Some(engine)),
        Err(e) => {
            println!("The content of engine file is invalid: {}", e);
            Ok(None)
        }
    }
}

fn set_engine(engine:Engine) -> Result<()>{
    let file_path = current_dir()?.join(ENGINE_FILE_NAME);
    std::fs::write(file_path,format!("{}",engine))?;
    Ok(())
}
