use internal::txn::{TxnId};

use collections::hashmap::{HashMap};

pub struct TableEntry {
  file_id: u32,
  value_pos: u32,
  value_size: u32,
  txnid: TxnId,
}

pub struct Table {
  map: HashMap<~[u8], TableEntry>,
}

impl Table {
  pub fn new() -> Table {
    Table{
      map: HashMap::new(),
    }
  }

  pub fn contains(&self, key: &~[u8]) -> bool {
    if !self.map.contains_key(key) {
      return false;
    }
    let entry = self.map.get(key);
    entry.value_size != 0
  }

  pub fn get<'a>(&'a self, key: &~[u8]) -> &'a TableEntry {
    self.map.get(key)
  }

  pub fn put(&mut self, txnid: TxnId, key: ~[u8], value_pos: u32, value_size: u32) {
    let entry = TableEntry{
      file_id: 0,
      value_pos: value_pos,
      value_size: value_size,
      txnid: txnid,
    };
    self.map.insert(key, entry);
  }

  pub fn delete(&mut self, txnid: TxnId, key: ~[u8], value_pos: u32) {
    let entry = TableEntry{
      file_id: 0,
      value_pos: value_pos,
      value_size: 0,
      txnid: txnid,
    };
    self.map.insert(key, entry);
  }
}
