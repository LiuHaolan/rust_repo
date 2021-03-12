use std::collections::HashMap;

pub struct KvStore {
    kv: HashMap<String, String>,
}

impl KvStore{
    pub fn new() -> KvStore {
        let res = KvStore { kv: HashMap::new(), };

        res
    }
    
    pub fn set(&mut self, key:String, value:String){
        self.kv.insert(key, value);
    }
    pub fn get(&self, key: String) -> Option<String> {
 //       let copy = key.clone();
        let s = self.kv.get(&key)?;
        let res: String = s.clone();

        Some(res)

    }
    pub fn remove(&mut self, key: String){
        self.kv.remove(&key);        
    }

}
