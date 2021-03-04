use std::collections::HashMap;

struct KvStore {
    kv: HashMap<String, String>,
}

impl KvStore{
    fn set(&mut self, key:String, value:String){
        self.kv.insert(key, value);
    }
    fn get(&self, key: String) -> Option<String> {
        let copy = key.clone();
        self.kv.get(&copy)
    }
    fn remove(&mut self, key: String){
        
    }

}
