//use collections::hashmap::{HashMap};

pub type TxnId = u64;

pub struct Txn {
  id: TxnId,
  kvs: Vec<(~[u8], ~[u8])>,
}

impl Txn {
  pub fn new(txnid: TxnId) -> Txn {
    Txn{
      id: txnid,
      kvs: Vec::new(),
    }
  }

  pub fn put(&mut self, key: ~[u8], value: ~[u8]) {
    self.kvs.push((key, value));
  }

  pub fn delete(&mut self, key: ~[u8]) {
    self.kvs.push((key, ~[]));
  }
}
