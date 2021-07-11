//#![deny(missing_docs)]
extern crate failure;
extern crate serde;
pub use error::{KvsError, Result};

mod error;
use std::{ io::{Read, Write}, path::PathBuf};
//pub type Result<T> = std::result::Result<T, Error>;
use serde::{Serialize, Deserialize};
use std::fs::OpenOptions;
#[derive(Serialize, Deserialize, Debug)]
enum Command{
    Set{key:String,value:String},
    Rm{key:String}
}


pub struct KvStore {
    /// the struct to simulate the key value store process
    filePath:PathBuf
}
impl KvStore {

    /// kvStore[key] -> value
    pub fn set(&mut self, key: String, value: String) -> Result<()>{
        let command = Command::Set{key,value};
        let mut serialized = serde_json::to_string(&command).expect("error when serializing");
        serialized = serialized + &"\n".to_string();
        
        let mut file = OpenOptions::new().append(true).open(&self.filePath).expect("error when write file in KvStore");
        file.write_all(serialized.as_bytes()).expect("error when write set command");
        Ok(())
    }
    /// get kvStore[key]
    pub fn get(&self, key: String) -> Result<Option<String>> {
        let mut ans : Option::<String> = None;
        for command in self.get_all_command(){
            match command{
                Command::Set { key:cur_key,value:cur_value  } => {
                    if key == cur_key {
                        ans = Some(cur_value);
                    }
                },
                Command::Rm {key:cur_key}=> {
                    if key == cur_key {
                        ans = None;
                    }
                },
                _ => unreachable!()
            }
        }
        Ok(ans)
    }
    /// remove kvStore[key]
    pub fn remove(&mut self, key: String) -> Result<()>{
        let contain_key : bool =  self.get_all_command().iter().any(| command| 
            {
                match command {
                Command::Set { key:cur_key,.. } => (key == cur_key.clone()),
                Command::Rm{..} => false,
            }} );
        if contain_key {
        let command = Command::Rm{key};
        let mut serialized = serde_json::to_string(&command).expect("error when serializing");
        serialized = serialized + &"\n".to_string();
        let mut file = OpenOptions::new().append(true).open(&self.filePath).expect("error when write file in KvStore");
        file.write_all(serialized.as_bytes()).expect("error when write set command");
        Ok(())
        }else{
            Err(KvsError::KeyNotFound)
        }
    }
    pub fn open(path: impl Into<PathBuf>) -> Result<KvStore>{
        let dir_path:PathBuf = path.into();
        let file_path = dir_path.join("task2.log");
        std::fs::create_dir_all(&dir_path).expect("error when create dir");
        // create log file
        std::fs::OpenOptions::new().read(true).write(true).create(true).open(&file_path).expect("error when open file in KvStore");
        Ok(KvStore{
            filePath: file_path
        })
    }

    fn get_all_command(&self) -> Vec<Command>{
        let mut file_string = String::new();
        let mut file  = std::fs::OpenOptions::new().read(true).open(&self.filePath).expect("error when read file in KvStore");
        file.read_to_string(&mut file_string).unwrap();
        let command_strs : Vec<&str> = file_string.trim().split("\n").collect();
        let mut commands :Vec<Command> = Vec::new();
        //println!("{:?}",&file_string);
        //println!("{:?}",&command_strs);
        for command_str in command_strs{
            if command_str.trim().is_empty(){
                continue;
            }
            commands.push(serde_json::from_str(&command_str).expect("wrong when deserlization"));
        }
        commands
    }
}
