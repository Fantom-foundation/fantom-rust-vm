use bigint::H256;
use tempdir::TempDir;
use trie::TrieMut;

use std::fs;
use std::path::Path;

// Database imports
use rkv::{Manager, Rkv, Store, StoreError, Value};

pub type RDB = std::sync::Arc<std::sync::RwLock<rkv::Rkv>>;

pub fn create_temporary_db() -> Result<(RDB, Store), StoreError> {
    let tempdir = TempDir::new("testing").unwrap();
    let root = tempdir.path();
    let created_arc = Manager::singleton().write().unwrap().get_or_create(root, Rkv::new)?;
    if let Ok(k) = created_arc.read() {
        if let Ok(a) = k.open_or_create("store") {
            return Ok((created_arc.clone(), a));
        }
    }
    Err(StoreError::DirectoryDoesNotExistError(root.into()))
}

pub fn create_persistent_db(path: &str, name: &str) -> Result<(RDB, Store), StoreError> {
    let root = path.to_string() + name + "/";
    fs::create_dir_all(root.clone())?;
    let root = Path::new(&root);
    let created_arc = Manager::singleton().write().unwrap().get_or_create(root, Rkv::new)?;
    if let Ok(k) = created_arc.read() {
        if let Ok(a) = k.open_or_create("store") {
            return Ok((created_arc.clone(), a));
        }
    }
    Err(StoreError::DirectoryDoesNotExistError(root.into()))
}

pub struct DB {
    root: H256,
    handle: RDB,
    database: Store,
}

impl DB {
    pub fn new_temporary(root: H256) -> DB {
        let (rkv, store) = create_temporary_db().unwrap();
        DB {
            root,
            handle: rkv,
            database: store,
        }
    }

    pub fn new_persistent(path: &str, name: &str, root: H256) -> DB {
        let (rkv, store) = create_persistent_db(path, name).unwrap();
        DB {
            root,
            handle: rkv,
            database: store,
        }
    }
}

impl TrieMut for DB {
    fn root(&self) -> H256 {
        self.root
    }

    fn insert(&mut self, key: &[u8], value: &[u8]) {
        match self.handle.read() {
            Ok(env_lock) => match env_lock.write() {
                Ok(mut writer) => {
                    let _result = writer.put(self.database, key, &Value::Blob(value));
                    let _result = writer.commit();
                }
                Err(_e) => {}
            },
            Err(_e) => {}
        }
    }

    fn delete(&mut self, key: &[u8]) {
        match self.handle.write() {
            Ok(env_lock) => match env_lock.write() {
                Ok(mut writer) => {
                    let _result = writer.delete(self.database, key);
                    let _result = writer.commit();
                }
                Err(_e) => {}
            },
            Err(_e) => {}
        }
    }
    fn get(&self, key: &[u8]) -> Option<Vec<u8>> {
        match self.handle.read() {
            Ok(env_lock) => match env_lock.read() {
                Ok(reader) => match reader.get(self.database, key) {
                    Ok(result) => match result {
                        Some(r) => {
                            let final_result: Vec<u8> = r.to_bytes().unwrap();
                            Some(final_result)
                        }
                        None => None,
                    },
                    Err(_e) => None,
                },
                Err(_e) => None,
            },
            Err(_e) => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_test_rkv() {
        let store = create_temporary_db();
        assert!(store.is_ok());
    }
}
