// use std::error::Error;

// pub trait StorageBackend {
//     fn set(&mut self, key: String, value: String);
//     fn get(&self, key: &str) -> Option<&String>;
//     fn delete(&mut self, key: &str);
//     fn serialize(&self) -> Result<Vec<u8>, Box<dyn Error>>;
//     fn deserialize(&mut self, data: &[u8]) -> Result<(), Box<dyn Error>>;
// }