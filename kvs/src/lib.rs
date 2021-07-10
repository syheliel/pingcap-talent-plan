//#![deny(missing_docs)]
extern crate failure;
use core::panic;
use std::{collections::HashMap, path::PathBuf};
use failure::Error;
pub type Result<T> = std::result::Result<T, Error>;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
enum Command{
    Set{key:String,value:String},
    Rm{key:String}
}
pub struct KvStore {
    /// the struct to simulate the key value store process
    map: HashMap<String, String>,
}
impl KvStore {
    /// retrun a new KvStore
    pub fn new() -> KvStore {
        KvStore {
            map: HashMap::new(),
        }
    }
    /// kvStore[key] -> value
    pub fn set(&mut self, key: String, value: String) -> Result<()>{
        panic!()
    }
    /// get kvStore[key]
    pub fn get(&self, key: String) -> Option<String> {
        self.map.get(&key).cloned()
    }
    /// remove kvStore[key]
    pub fn remove(&mut self, key: String) {
        self.map.remove(&key);
    }
    pub fn open(path: impl Into<PathBuf>) -> Result<KvStore>{
        panic!()
    }
}
