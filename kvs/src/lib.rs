//#![deny(missing_docs)]
extern crate failure;
extern crate serde;
pub use error::{KvsError, Result};

mod error;
use std::{ io::{Read, Write}, path::PathBuf};
//pub type Result<T> = std::result::Result<T, Error>;
use serde::{Serialize, Deserialize};
use std::fs::{OpenOptions};
use std::collections::{HashMap, HashSet};
use std::io::{SeekFrom, Seek};

const COMPACTION_THRESHOLD: u64 = 1024 * 1024;
#[derive(Serialize, Deserialize, Debug)]
enum Command{
    Set{key:String,value:String},
    Rm{key:String}
}
struct Pointer{
    /// record log information of every key
    pos:usize,
    len:usize
}

pub struct KvStore {
    /// the struct to simulate the key value store process
    file_path:PathBuf,
    pointer_map:HashMap<String,Pointer>,
    next_pos:usize //record position for next command
}


impl KvStore {

    /// kvStore[key] -> value
    pub fn set(&mut self, key: String, value: String) -> Result<()>{
        let command = Command::Set{key:key.clone(),value};
        let command_len = self.append_command(&command)?;

        // 加入缓存
        let pointer = Pointer{pos:self.next_pos,len:command_len};
        self.next_pos += pointer.len + 1; // 加上当前序列化内容的长度 + 分隔符长度
        self.pointer_map.insert(key,pointer);
        if self.next_pos > COMPACTION_THRESHOLD as usize {
            self.compact();
        }
        Ok(())
    }


    /// get kvStore[key]
    pub fn get(&self, key: String) -> Result<Option<String>> {
        let mut ans: Option<String> = None;
        if let Some(pointer) = self.pointer_map.get(&key){
            let command =  self.get_one_command(pointer)?;
            match command {
                // pointer指向的总是Command::Set类型
                Command::Set {value,..} => ans = Some(value),
                _ => unreachable!(),
            }
        }
        Ok(ans)
    }
    /// remove kvStore[key]
    pub fn remove(&mut self, key: String) -> Result<()>{
        if self.pointer_map.contains_key(&key) {
            //先写入日志
            let command = Command::Rm{key:key.clone()};
            self.append_command(&command)?;
            //之后更改哈希
            self.pointer_map.remove(&key);
            Ok(())
        }else{
            Err(KvsError::KeyNotFound)
        }
    }
    pub fn open(path: impl Into<PathBuf>) -> Result<KvStore>{
        let dir_path:PathBuf = path.into();
        std::fs::create_dir_all(&dir_path).expect("error when create dir");
        let file_path = dir_path.join("task2.log");

        // create log file
        std::fs::OpenOptions::new().read(true).write(true).create(true).open(&file_path).expect("error when open file in KvStore");


        let mut kv_store = KvStore{
            file_path,
            pointer_map:HashMap::new(),
            next_pos:0
        };
        kv_store.compact()?;
        // kv_store.rebuild_map()?;
        Ok(kv_store)

    }

    fn get_all_command(&self) -> Vec<Command>{
        // 读取文件
        let mut file  = std::fs::OpenOptions::new().read(true).open(&self.file_path).expect("error when read file in KvStore");
        let mut file_string = String::new();
        file.read_to_string(&mut file_string).unwrap();

        // 处理文本
        let command_strs : Vec<&str> = file_string.trim().split('\n').collect();
        let mut commands  = Vec::new();
        for command_str in command_strs{
            if command_str.trim().is_empty(){
                continue;
            }
            commands.push(serde_json::from_str(&command_str).expect("wrong when deserialization"));
        }
        commands
    }

    fn get_one_command(&self,pointer:&Pointer) -> Result<Command>{
        let mut file  = std::fs::OpenOptions::new().read(true).open(&self.file_path).expect("error when read file in KvStore");

        // seek for start position
        file.seek(SeekFrom::Start(pointer.pos as u64))?;
        let mut buffer = Vec::new();
        // read exact amount of bytes
        let mut handle = file.take(pointer.len as u64);
        handle.read_to_end(&mut buffer)?;
        let string = String::from_utf8(buffer)?;
        let command = serde_json::from_str(&string).expect("wrong when deserialization");
        Ok(command)
    }

    /// append command to the log's end. return size of command if success
    fn append_command(&self,command:&Command) -> Result<usize>{
        let serialized = serde_json::to_string(&command).expect("error when serializing");
        let mut file = OpenOptions::new().append(true).open(&self.file_path).expect("error when write file in KvStore");
        file.write_all(serialized.as_bytes()).expect("error when write command");
        file.write_all("\n".as_bytes()).expect("error when write delimiter");
        Ok(serialized.as_bytes().len())
    }

    fn rebuild_map(&mut self) -> Result<()>{
        for command in &self.get_all_command(){
           match command {
               Command::Set {key,..} => {
                   let len = serde_json::to_string(&command)?.as_bytes().len();
                   let pointer = Pointer{len,pos:self.next_pos};
                   self.pointer_map.insert(key.clone(), pointer);
                   self.next_pos += len + 1;
               },
               Command::Rm {key} => {
                   self.pointer_map.remove(key);
                   let len = serde_json::to_string(&command)?.as_bytes().len();
                   self.next_pos += len + 1;
               }
           }
        }
        Ok(())
    }

    /// compact log and then rebuild pointer_map
    fn compact(&mut self) -> Result<()>{
        let mut commands = self.get_all_command();
        commands.reverse();
        let mut key_set = HashSet::new();
        // 倒序遍历commands，找到对于某个key最后出现的command
        let mut valid_commands = Vec::new();
        for command in &commands{
            match command{
                Command::Set {key,..} => {
                    if !key_set.contains(key) {
                        valid_commands.push(command);
                        key_set.insert(key);
                    }
                }
                Command::Rm {key} => {
                    if !key_set.contains(key) {
                        valid_commands.push(command);
                        key_set.insert(key);
                    }
                }
            }
        }

        self.next_pos = 0;
        std::fs::remove_file(&self.file_path)?;
        std::fs::File::create(&self.file_path)?;
        valid_commands.reverse();
        for command in valid_commands{
            self.append_command(command)?;
        }
        self.rebuild_map()?;
        Ok(())
    }
}
