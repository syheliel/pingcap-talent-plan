pub struct KvStore{

}
impl KvStore {
	pub fn new() -> KvStore{
		KvStore {}
	}
	pub fn set(&self,key:String,value:String){
		panic!()
	}
	pub fn get(&self,key:String) -> Option<String>{
		panic!()
		//Some("s".to_string())
	}
	pub fn remove(&self,key:String){
		panic!()
	}
	
}