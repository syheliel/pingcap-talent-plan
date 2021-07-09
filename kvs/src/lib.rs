//#![deny(missing_docs)]
use std::collections::HashMap;

pub struct KvStore{
	/// the struct to simulate the key value store process
	map:HashMap<String,String>,
}
impl KvStore {
	/// retrun a new KvStore
	pub fn new() -> KvStore{
		KvStore {
			map:HashMap::new()
		}
	}
	/// kvStore[key] -> value
	pub fn set(&mut self,key:String,value:String){
		self.map.insert(key,value);
	}
	/// get kvStore[key]
	pub fn get(&self,key:String) -> Option<String>{
		self.map.get(&key).cloned()
	}
	/// remove kvStore[key]
	pub fn remove(&mut self,key:String){
		self.map.remove(&key);
	}
}