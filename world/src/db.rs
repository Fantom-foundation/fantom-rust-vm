use bigint::{H256, U256};
use tempdir::TempDir;
use trie::TrieMut;

// Database imports
use rkv::{Manager, Rkv, Store, StoreError, Value};

type RDB = std::sync::Arc<std::sync::RwLock<rkv::Rkv>>;

fn create_temporary_db() -> Result<(RDB, Store), StoreError> {
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

pub struct WorldDB {
    root: H256,
    handle: RDB,
    database: Store,
}

impl WorldDB {
    pub fn new(root: H256) -> WorldDB {
        let (rkv, store) = create_temporary_db().unwrap();
        WorldDB {
            root: root,
            handle: rkv,
            database: store,
        }
    }
}

impl TrieMut for WorldDB {
    fn root(&self) -> H256 {
        self.root
    }

    fn insert(&mut self, key: &[u8], value: &[u8]) {
        match self.handle.read() {
            Ok(env_lock) => match env_lock.write() {
                Ok(mut writer) => {
                    writer.put(self.database, key, &Value::Blob(value));
                    writer.commit();
                }
                Err(e) => {}
            },
            Err(e) => {}
        }
    }

    fn delete(&mut self, key: &[u8]) {
        match self.handle.write() {
            Ok(env_lock) => match env_lock.write() {
                Ok(mut writer) => {
                    writer.delete(self.database, key);
                    writer.commit();
                }
                Err(e) => {}
            },
            Err(e) => {}
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
